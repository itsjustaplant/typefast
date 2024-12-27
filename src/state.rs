use crate::constants::Screen;

#[derive(Debug, Default)]
pub struct State {
    pub is_running: bool,
    pub screen: Screen,
    pub error: String,
    pub counter: i32,
}

impl State {
    pub fn new() -> Self {
        State::default()
    }
    pub fn get_is_running(&self) -> bool {
        self.is_running
    }

    pub fn set_is_running(&mut self, is_running: bool) {
        self.is_running = is_running;
    }

    pub fn get_error(&self) -> &String {
        &self.error
    }

    pub fn set_error(&mut self, error: String) {
        self.error = error;
    }

    pub fn get_screen(&self) -> &Screen {
        &self.screen
    }

    pub fn set_screen(&mut self, screen: Screen) {
        self.screen = screen;
    }

    pub fn get_counter(&self) -> i32 {
        self.counter
    }

    pub fn set_counter(&mut self, counter: i32) {
        self.counter = counter
    }
}
