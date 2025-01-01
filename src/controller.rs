use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::prelude::{Backend, Terminal};
use thiserror::Error;

use crate::constants::{Action, Screen, COUNTDOWN_DURATION, GAME_DURATION, TEST_WORDS};
use crate::filesystem::{create_config_folder, get_app_config_path};
use crate::state::State;
use crate::util::{calculate_char_speed, calculate_word_speed};
use crate::view::View;

type DynamicError = Box<dyn std::error::Error>;

#[derive(Default)]
pub struct Controller {
    pub state: State,
    timer_running: Arc<AtomicBool>,
    remaining_time: Arc<Mutex<u64>>,
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

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Init => {
                self.state.set_is_running(true);
                let parsed_words = TEST_WORDS.map(|word| word.to_string());
                // self.state.set_words(&parsed_words);
                let parsed_paragraph = parsed_words.map(|word| word.to_lowercase()).join(" ");
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
                    self.state.set_error(user_input.to_string());
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
            Action::ChangeScene(screen) => {
                match screen {
                    Screen::CountDown => {
                        self.setup_timer(COUNTDOWN_DURATION);
                        self.state.set_next_screen(Screen::Main);
                    }
                    Screen::Main => {
                        self.setup_timer(GAME_DURATION);
                        self.state.set_next_screen(Screen::Menu);
                    }
                    Screen::Menu => {
                        self.state.set_next_screen(Screen::CountDown);
                    }
                }
                self.state.set_screen(screen);
            }
            Action::Empty => {}
        }
    }

    pub fn handle_key_stroke(&mut self, key_code: KeyCode) -> Action {
        match key_code {
            KeyCode::Esc => Action::Exit,
            KeyCode::Enter => {
                if self.state.get_screen() == &Screen::Menu {
                    Action::ChangeScene(Screen::CountDown)
                } else {
                    Action::ChangeScene(Screen::Main)
                }
            }
            KeyCode::Char(user_input) => Action::CharInput(user_input),
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
        self.handle_action(Action::Init);
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), DynamicError> {
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
                    let action = match self.state.get_next_screen() {
                        Screen::Main => Action::ChangeScene(Screen::Main),
                        Screen::Menu => Action::ChangeScene(Screen::Menu),
                        Screen::CountDown => Action::ChangeScene(Screen::CountDown),
                    };
                    drop(time);
                    self.timer_running.store(false, Ordering::SeqCst);
                    self.handle_action(action);
                }
            }
            thread::sleep(Duration::from_millis(5));
        }
        Ok(())
    }
}
