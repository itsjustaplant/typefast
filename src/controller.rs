use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::prelude::{Backend, Terminal};
use thiserror::Error;

use crate::constants::DB_NAME;
use crate::filesystem::{create_config_folder, get_app_config_path};
use crate::state::State;
use crate::util::{calculate_char_speed, calculate_word_speed};
use crate::view::View;
use crate::{client::Client, filesystem};
use crate::{
    constants::{Action, Page, COUNTDOWN_DURATION, GAME_DURATION},
    filesystem::get_words,
};

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

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Init => {
                self.state.set_is_running(true);
                let parsed_words = get_words()
                    .iter()
                    .map(|word| word.to_string())
                    .collect::<Vec<String>>();
                // self.state.set_words(&parsed_words);
                let parsed_paragraph = parsed_words
                    .iter()
                    .map(|word| word.to_lowercase())
                    .collect::<Vec<String>>()
                    .join(" ");
                self.state.set_paragraph(parsed_paragraph);
            }
            Action::Exit => {
                self.state.set_is_running(false);
                self.exit().expect("Could not exit");
            }
            Action::CharInput(user_input) => {
                let current_position = self.state.get_position() as usize;
                if let Some(current_character) =
                    self.state.get_paragraph().chars().nth(current_position)
                {
                    // self.state.set_error(user_input.to_string());
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
                        self.handle_action(Action::GetRecords);
                        self.stop_timer();
                    }
                    Page::GameResult => {
                        self.stop_timer();
                        self.handle_action(Action::PostRecord);
                        self.handle_action(Action::ChangePage(Page::Menu));
                    }
                }
                self.state.set_page(page);
            }
            Action::GetRecords => match self.client.get_records() {
                Ok(records) => {
                    self.state.set_records(records);
                }
                Err(_) => {
                    self.state.set_error("Could not get records".to_string());
                }
            },
            Action::PostRecord => {
                let wpm = self.state.get_word_speed();
                let cpm = self.state.get_char_speed();
                let date = "date";
                if self.client.create_record(wpm, cpm, date.to_string()).is_err() {
                    self.state.set_error("Could not Save record".to_string());
                }
            }
            Action::MenuAction => {
                let menu_index = self.state.get_menu_index();
                let new_index = if menu_index <= 0 { 1 } else { 0 };
                self.state.set_menu_index(new_index);
            }
            Action::Empty => {}
        }
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
                    self.handle_action(action);
                }
            }
        }
        Ok(())
    }

    pub fn init_controller(&mut self) -> Result<(), DynamicError> {
        let app_config_path = get_app_config_path()?;
        create_config_folder(&app_config_path)?;

        filesystem::create_config_folder(&app_config_path)?;
        self.client.open_connection(app_config_path, DB_NAME)?;
        self.client.create_records_table()?;

        self.handle_action(Action::Init);
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), DynamicError> {
        self.client.close_connection()?;
        Ok(())
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), DynamicError> {
        self.init_controller()?;

        while self.state.get_is_running() {
            let _ = self
                .handle_events()
                .map_err(ControllerError::HandleEventError);
            View::draw(terminal, &self.state)?;

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
                    self.handle_action(action);
                }
            }
            thread::sleep(Duration::from_millis(5));
        }
        Ok(())
    }
}
