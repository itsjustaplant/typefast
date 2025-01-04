use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::prelude::{Backend, Terminal};
use thiserror::Error;

use crate::client::{Client, ClientError};
use crate::constants::{Action, Page, COUNTDOWN_DURATION, DB_NAME, GAME_DURATION};
use crate::filesystem::{create_config_folder, get_app_config_path, get_words, FileSystemError};
use crate::state::State;
use crate::util::{calculate_char_speed, calculate_word_speed, get_current_datetime};
use crate::view::{View, ViewError};

type DynamicError = Box<dyn std::error::Error>;

#[derive(Default)]
pub struct Controller {
    pub state: State,
    timer_running: Arc<AtomicBool>,
    remaining_time: Arc<Mutex<u64>>,
    client: Client,
}

#[derive(Error, Debug)]
pub enum ControllerError {
    #[error("Database client error: {0}")]
    ClientError(#[from] ClientError),
    #[error("File system error: {0}")]
    FileSystemError(#[from] FileSystemError),
    #[error("View error: {0}")]
    ViewError(#[from] ViewError),
    #[error("Encountered with error while handling keyboard events: {0}")]
    HandleEventError(DynamicError),
}

impl Controller {
    pub fn new() -> Self {
        Self {
            state: State::new(),
            timer_running: Arc::new(AtomicBool::new(false)),
            remaining_time: Arc::new(Mutex::new(0)),
            client: Client::default(),
        }
    }

    pub fn setup_timer(&mut self, duration: u64) {
        if self.timer_running.load(Ordering::SeqCst) {
            return;
        }

        self.timer_running.store(true, Ordering::SeqCst);
        let timer_running = Arc::clone(&self.timer_running);
        let remaining_time = Arc::clone(&self.remaining_time);

        *remaining_time.lock().unwrap() = duration;

        thread::spawn(move || {
            for _ in 0..duration {
                if !timer_running.load(Ordering::SeqCst) {
                    timer_running.store(false, Ordering::SeqCst);
                    break;
                }
                thread::sleep(Duration::from_secs(1));
                let mut time = remaining_time.lock().unwrap();
                if *time > 0 {
                    *time -= 1;
                }
            }
        });
    }

    pub fn stop_timer(&mut self) {
        self.timer_running.store(false, Ordering::SeqCst);
    }

    pub fn handle_action(&mut self, action: Action) -> Result<(), ControllerError> {
        match action {
            Action::Init => {
                self.state.set_is_running(true);
                let parsed_words = get_words()
                    .iter()
                    .map(|word| word.to_string())
                    .collect::<Vec<String>>();
                let parsed_paragraph = parsed_words
                    .iter()
                    .map(|word| word.to_lowercase())
                    .collect::<Vec<String>>()
                    .join(" ");
                self.state.set_paragraph(parsed_paragraph);
            }
            Action::Exit => {
                self.state.set_is_running(false);
                self.exit()?;
            }
            Action::CharInput(user_input) => {
                let current_position = self.state.get_position() as usize;
                if let Some(current_character) =
                    self.state.get_paragraph().chars().nth(current_position)
                {
                    if current_character == user_input {
                        self.state.set_position((current_position + 1) as i32);

                        if current_character == ' ' {
                            let word_count = self.state.get_word_count() + 1;
                            self.state.set_word_count(word_count);
                        } else {
                            let char_count = self.state.get_char_count() + 1;
                            self.state.set_char_count(char_count);
                        }
                    }
                } else {
                    self.state.set_is_running(false);
                }
            }
            Action::ChangePage(page) => {
                match page {
                    Page::CountDown => {
                        self.setup_timer(COUNTDOWN_DURATION);
                        self.state.set_next_page(Page::Game);
                    }
                    Page::Game => {
                        self.setup_timer(GAME_DURATION);
                        self.state.set_next_page(Page::GameResult);
                    }
                    Page::Menu => {
                        self.state.set_next_page(Page::CountDown);
                    }
                    Page::Records => {
                        self.handle_action(Action::GetRecords).ok();
                        self.stop_timer();
                    }
                    Page::GameResult => {
                        self.stop_timer();
                        self.handle_action(Action::PostRecord).ok();
                        self.handle_action(Action::ChangePage(Page::Menu)).ok();
                    }
                }
                self.state.set_page(page);
            }
            Action::GetRecords => {
                let records = self.client.get_records()?;
                self.state.set_records(records);
            }

            Action::PostRecord => {
                let wpm = self.state.get_word_speed();
                let cpm = self.state.get_char_speed();
                let date = get_current_datetime();
                self.client.create_record(wpm, cpm, date)?;
            }
            Action::MenuAction => {
                let menu_index = self.state.get_menu_index();
                let new_index = if menu_index <= 0 { 1 } else { 0 };
                self.state.set_menu_index(new_index);
            }
            Action::Empty => {}
        }
        Ok(())
    }

    pub fn handle_key_stroke(&mut self, key_code: KeyCode) -> Action {
        match key_code {
            KeyCode::Esc => {
                self.stop_timer();
                match self.state.get_page() {
                    Page::Menu => Action::Exit,
                    _ => Action::ChangePage(Page::Menu),
                }
            }
            KeyCode::Enter => match self.state.get_page() {
                Page::Menu => {
                    if self.state.get_menu_index() == 0 {
                        Action::ChangePage(Page::CountDown)
                    } else {
                        Action::ChangePage(Page::Records)
                    }
                }
                Page::GameResult => Action::ChangePage(Page::Menu),
                _ => Action::Empty,
            },
            KeyCode::Char(user_input) => match self.state.get_page() {
                Page::Game => Action::CharInput(user_input),
                _ => Action::Empty,
            },
            KeyCode::Down => Action::MenuAction,
            KeyCode::Up => Action::MenuAction,
            _ => Action::Empty,
        }
    }

    pub fn handle_events(&mut self) -> Result<(), DynamicError> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let action = self.handle_key_stroke(key.code);
                    self.handle_action(action).ok();
                }
            }
        }
        Ok(())
    }

    pub fn init_controller(&mut self) -> Result<(), ControllerError> {
        let app_config_path = get_app_config_path()?;
        create_config_folder(&app_config_path)?;

        self.client.open_connection(app_config_path, DB_NAME)?;
        self.client.create_records_table()?;

        self.handle_action(Action::Init).ok();
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), ControllerError> {
        self.client.close_connection()?;
        Ok(())
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), ControllerError> {
        // this line sets the global
        self.init_controller()
            .map_err(|e| self.state.set_error(e.to_string()))
            .ok();

        while self.state.get_is_running() {
            let _ = self.handle_events().map_err(|e| {
                self.state
                    .set_error(ControllerError::HandleEventError(e).to_string())
            });
            View::draw(terminal, &self.state).map_err(|e| {
                self.state.set_error(ControllerError::ViewError(e).to_string())
            }).ok();

            if self.timer_running.load(Ordering::SeqCst) {
                let time = self.remaining_time.lock().unwrap();
                let time_value = *time;
                let current_time_value = self.state.get_timer();

                if time_value != current_time_value {
                    let word_speed = calculate_word_speed(
                        self.state.word_count,
                        GAME_DURATION - current_time_value,
                    );
                    let char_speed = calculate_char_speed(
                        self.state.char_count,
                        GAME_DURATION - current_time_value,
                    );
                    self.state.set_word_speed(word_speed);
                    self.state.set_char_speed(char_speed);
                    self.state.set_timer(time_value);
                }

                if time_value == 0 {
                    let action = match self.state.get_next_page() {
                        Page::Game => Action::ChangePage(Page::Game),
                        Page::Menu => Action::ChangePage(Page::Menu),
                        Page::CountDown => Action::ChangePage(Page::CountDown),
                        Page::Records => Action::Empty,
                        Page::GameResult => Action::ChangePage(Page::GameResult),
                    };
                    drop(time);
                    self.stop_timer();
                    self.handle_action(action).ok();
                }
            }
            thread::sleep(Duration::from_millis(5));
        }
        Ok(())
    }
}
