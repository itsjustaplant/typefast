use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::prelude::{Backend, Terminal};

use crate::constants::Action;
use crate::filesystem::{self, get_app_config_path};
use crate::state::State;
use crate::view::View;

pub struct Controller {
    pub state: State,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            state: State::new(),
        }
    }

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Init => self.state.set_is_running(true),
            Action::Exit => {
                self.state.set_is_running(false);
                self.exit().expect("Could not exit");
            }
            Action::Empty => {
                let current_counter = self.state.get_counter();
                self.state.set_counter(current_counter + 1);
            }
        }
    }

    pub fn handle_key_stroke(&mut self, key_code: KeyCode) -> Action {
        match key_code {
            KeyCode::Esc => Action::Exit,
            _ => Action::Empty,
        }
    }

    pub fn handle_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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

    pub fn init_controller(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let app_config_path = get_app_config_path()?;

        filesystem::create_config_folder(&app_config_path)?;
        self.handle_action(Action::Init);
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.init_controller()?;

        while self.state.get_is_running() {
            self.handle_events()?;
            View::draw(terminal, &self.state)?;
        }
        Ok(())
    }
}
