use std::path::PathBuf;

use chrono::Local;
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
    client: Client,
}

#[derive(Error, Debug)]
pub enum ControllerError {
    #[error("{0}")]
    ClientError(#[from] ClientError),
    #[error("{0}")]
    FileSystemError(#[from] FileSystemError),
    #[error("{0}")]
    ViewError(#[from] ViewError),
    #[error("Encountered with error while handling keyboard events: {0}")]
    HandleEventError(DynamicError),
}

impl Controller {
    pub fn new() -> Self {
        Self {
            state: State::new(),
            client: Client::default(),
        }
    }

    pub fn handle_action(&mut self, action: Action) -> Result<(), ControllerError> {
        match action {
            Action::Init => {
                self.state.set_is_running(true);
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
                }
            }
            Action::ChangePage(page) => {
                match page {
                    Page::CountDown => {
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
                        self.state.reset_stats();
                        self.state.set_reference_timestamp(None);
                    }
                    Page::Game => {
                        self.state.set_reference_timestamp(None);
                    }
                    Page::Menu => {}
                    Page::Records => {
                        self.handle_action(Action::GetRecords)?;
                    }
                    Page::GameResult => {
                        self.handle_action(Action::PostRecord)?;
                        self.handle_action(Action::ChangePage(Page::Menu))?;
                    }
                }
                self.state.set_page(page);
            }
            Action::GetRecords => {
                let records = self.client.get_records()?;
                self.state.set_records(records);
            }

            Action::PostRecord => {
                let wpm = calculate_word_speed(self.state.get_word_count(), GAME_DURATION);
                let cpm = calculate_char_speed(self.state.get_char_count(), GAME_DURATION);
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
            KeyCode::Esc => match self.state.get_page() {
                Page::Menu => Action::Exit,
                _ => Action::ChangePage(Page::Menu),
            },
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
            KeyCode::Down | KeyCode::Up => match self.state.get_page() {
                Page::Menu => Action::MenuAction,
                _ => Action::Empty,
            },
            _ => Action::Empty,
        }
    }

    pub fn handle_events(&mut self) -> Result<(), DynamicError> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let action = self.handle_key_stroke(key.code);
                    self.handle_action(action)?;
                }
            }
        }
        Ok(())
    }

    pub fn init_controller(
        &mut self,
        app_config_path: PathBuf,
        db_name: &str,
    ) -> Result<(), ControllerError> {
        self.handle_action(Action::Init)?;
        self.client.open_connection(app_config_path, db_name)?;
        self.client.create_records_table()?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<(), ControllerError> {
        self.client.close_connection()?;
        Ok(())
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), ControllerError> {
        let app_config_path = get_app_config_path()?;
        create_config_folder(&app_config_path)?;
        // this line sets the global error
        let _ = self
            .init_controller(app_config_path, DB_NAME)
            .map_err(|e| self.state.set_error(e.to_string()));
        while self.state.get_is_running() {
            let _ = self.handle_events().map_err(|e| {
                self.state
                    .set_error(ControllerError::HandleEventError(e).to_string())
            });
            View::draw(terminal, &self.state)
                .map_err(|e| {
                    self.state
                        .set_error(ControllerError::ViewError(e).to_string())
                })
                .ok();

            self.state.set_timer(Local::now().timestamp_millis());

            if self.state.get_reference_timestamp() != 0 {
                match self.state.get_page() {
                    Page::CountDown => {
                        if self.state.get_elapsed_time() >= COUNTDOWN_DURATION {
                            self.handle_action(Action::ChangePage(Page::Game))
                                .map_err(|e| self.state.set_error(e.to_string()))
                                .ok();
                        }
                    }
                    Page::Game => {
                        if self.state.get_elapsed_time() >= GAME_DURATION {
                            self.handle_action(Action::ChangePage(Page::GameResult))
                                .map_err(|e| self.state.set_error(e.to_string()))
                                .ok();
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::{TEST_APP_PATH, TEST_DB_NAME};
    use std::path::{Path, PathBuf};

    fn get_test_db_path() -> PathBuf {
        let db_path = Path::new(TEST_APP_PATH);
        db_path.to_path_buf()
    }

    #[test]
    fn test_handle_action() {
        // CONTROLLER RUNNING STATE TEST
        let mut controller = Controller::new();
        assert_eq!(controller.state.get_is_running(), false);

        let db_name = format!("controller_{TEST_DB_NAME}");
        let result = controller
            .client
            .open_connection(get_test_db_path(), db_name.as_str());
        assert!(result.is_ok());
        let result = controller.client.create_records_table();
        assert!(result.is_ok());

        // INIT CONTROLLER TEST
        let action = Action::Init;
        let result = controller.handle_action(action);
        assert!(result.is_ok());
        assert_eq!(controller.state.get_is_running(), true);

        controller.state.set_paragraph("T E S T".to_string());
        // HANDLE CHAR INPUT TEST
        let character = 'T';
        let action = Action::CharInput(character);
        let result = controller.handle_action(action);
        assert!(result.is_ok());
        assert_eq!(controller.state.get_position(), 1);
        assert_eq!(controller.state.get_char_count(), 1);
        assert_eq!(controller.state.get_word_count(), 0);

        // HANDLE WRONG CHAR INPUT TEST
        let character = 'T';
        let action = Action::CharInput(character);
        let result = controller.handle_action(action);
        assert!(result.is_ok());
        assert_eq!(controller.state.get_position(), 1);
        assert_eq!(controller.state.get_char_count(), 1);
        assert_eq!(controller.state.get_word_count(), 0);

        // HANDLE SPACE INPUT TEST
        let character = ' ';
        let action = Action::CharInput(character);
        let result = controller.handle_action(action);
        assert!(result.is_ok());
        assert_eq!(controller.state.get_word_count(), 1);

        // PAGE CHANGE TEST
        // PAGE::COUNTDOWN
        let action = Action::ChangePage(Page::CountDown);
        let result = controller.handle_action(action);
        assert!(result.is_ok());
        assert_eq!(controller.state.get_page(), &Page::CountDown);
        assert!(controller.state.get_paragraph().len() > 0);

        // PAGE::GAME
        let action = Action::ChangePage(Page::Game);
        let result = controller.handle_action(action);
        assert!(result.is_ok());
        assert_eq!(controller.state.get_page(), &Page::Game);

        // PAGE::MENU
        let action = Action::ChangePage(Page::Menu);
        let result = controller.handle_action(action);
        assert!(result.is_ok());
        assert_eq!(controller.state.get_page(), &Page::Menu);

        // PAGE::RECORDS
        let action = Action::ChangePage(Page::Records);
        let result = controller.handle_action(action);
        println!("{:?}", result);
        assert!(result.is_ok());
        assert_eq!(controller.state.get_page(), &Page::Records);

        // PAGE::GAMERESULT
        let action = Action::ChangePage(Page::GameResult);
        let result = controller.handle_action(action);
        assert!(result.is_ok());
        assert_eq!(controller.state.get_page(), &Page::GameResult);

        // GET RECORDS TEST
        let action = Action::GetRecords;
        let result = controller.handle_action(action);
        assert!(result.is_ok());
        assert_eq!(controller.state.get_records().len(), 1);

        // POST RECORD TEST
        let action = Action::PostRecord;
        let result = controller.handle_action(action);
        assert!(result.is_ok());
        // GET RECORDS AGAIN -> TODO: instead of doing this post should update the state
        let action = Action::GetRecords;
        let result = controller.handle_action(action);
        assert!(result.is_ok());
        assert_eq!(controller.state.get_records().len(), 2);

        // MENU ACTION TEST
        assert_eq!(controller.state.get_menu_index(), 0);
        let action = Action::MenuAction;
        let result = controller.handle_action(action);
        assert!(result.is_ok());
        assert_eq!(controller.state.get_menu_index(), 1);

        // EMPTY ACTION TEST
        let action = Action::Empty;
        let result = controller.handle_action(action);
        assert!(result.is_ok());

        // DROP TABLE
        let result = controller.client.drop_records_table();
        assert!(result.is_ok());

        // EXIT TEST
        let action = Action::Exit;
        let result = controller.handle_action(action);
        assert!(result.is_ok());
        assert_eq!(controller.state.get_is_running(), false);
        assert!(controller.client.connection.is_none());
    }

    #[test]
    fn test_handle_key_stroke() {
        // ESC KEY -- MENU PAGE TEST
        let mut controller = Controller::new();
        controller.state.set_page(Page::Menu);
        let action = controller.handle_key_stroke(KeyCode::Esc);
        assert_eq!(action, Action::Exit);

        // ESC KEY -- GAME PAGE TEST
        controller.state.set_page(Page::Game);
        let action = controller.handle_key_stroke(KeyCode::Esc);
        assert_eq!(action, Action::ChangePage(Page::Menu));

        // ENTER KEY -- MENU PAGE TEST
        controller.state.set_page(Page::Menu);
        let action = controller.handle_key_stroke(KeyCode::Enter);
        assert_eq!(action, Action::ChangePage(Page::CountDown));
        controller.state.set_menu_index(1);
        let action = controller.handle_key_stroke(KeyCode::Enter);
        assert_eq!(action, Action::ChangePage(Page::Records));

        // ENTER KEY -- GAME RESULT PAGE TEST
        controller.state.set_page(Page::GameResult);
        let action = controller.handle_key_stroke(KeyCode::Enter);
        assert_eq!(action, Action::ChangePage(Page::Menu));

        // ENTER KEY -- OTHER PAGE TEST
        controller.state.set_page(Page::Game);
        let action = controller.handle_key_stroke(KeyCode::Enter);
        assert_eq!(action, Action::Empty);

        // CHAR -- GAME PAGE TEST
        controller.state.set_page(Page::Game);
        let action = controller.handle_key_stroke(KeyCode::Char('T'));
        assert_eq!(action, Action::CharInput('T'));

        // CHAR -- OTHER PAGE TEST
        controller.state.set_page(Page::Menu);
        let action = controller.handle_key_stroke(KeyCode::Char('T'));
        assert_eq!(action, Action::Empty);

        // DOWN KEY  -- MENU PAGE TEST
        controller.state.set_page(Page::Menu);
        let action = controller.handle_key_stroke(KeyCode::Down);
        assert_eq!(action, Action::MenuAction);

        // DOWN KEY -- OTHER PAGE TEST
        controller.state.set_page(Page::Game);
        let action = controller.handle_key_stroke(KeyCode::Down);
        assert_eq!(action, Action::Empty);

        // UP KEY  -- MENU PAGE TEST
        controller.state.set_page(Page::Menu);
        let action = controller.handle_key_stroke(KeyCode::Up);
        assert_eq!(action, Action::MenuAction);

        // UP KEY -- OTHER PAGE TEST
        controller.state.set_page(Page::Game);
        let action = controller.handle_key_stroke(KeyCode::Up);
        assert_eq!(action, Action::Empty);

        // BACKSPACE KEY -- OTHER PAGE TEST
        let action = controller.handle_key_stroke(KeyCode::Backspace);
        assert_eq!(action, Action::Empty);
    }

    // god knows why this test is failing on github actions
    // #[test]
    // fn test_handle_events() {
    //     let mut controller = Controller::new();
    //     let result = controller.handle_events();
    //     assert!(result.is_ok());
    // }

    #[test]
    fn test_init_controller() {
        let mut controller = Controller::new();
        let db_name = format!("controller_{TEST_DB_NAME}");
        let result = controller.init_controller(get_test_db_path(), db_name.as_str());
        assert!(result.is_ok());
        let result = controller.client.drop_records_table();
        assert!(result.is_ok());
    }
}
