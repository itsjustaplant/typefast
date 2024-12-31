use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::prelude::{Backend, Terminal};
use thiserror::Error;

use crate::constants::{Action, TEST_WORDS};
use crate::filesystem::{create_config_folder, get_app_config_path};
use crate::state::State;
use crate::view::View;

type DynamicError = Box<dyn std::error::Error>;

#[derive(Default)]
pub struct Controller {
    pub state: State,
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
        }
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
                    }
                } else {
                    self.state.set_is_running(false);
                }
            }
            Action::Empty => {}
        }
    }

    pub fn handle_key_stroke(&mut self, key_code: KeyCode) -> Action {
        match key_code {
            KeyCode::Esc => Action::Exit,
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
        }
        Ok(())
    }
}
