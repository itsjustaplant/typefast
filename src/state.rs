use crate::constants::Screen;

#[derive(Debug, Default)]
pub struct State {
    pub is_running: bool,
    pub screen: Screen,
    pub next_screen: Screen,
    pub error: String,
    pub position: i32,
    pub paragraph: String,
    pub timer: String,
    pub duration: u64,
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

    pub fn get_next_screen(&self) -> &Screen {
        &self.next_screen
    }
    pub fn set_next_screen(&mut self, next_screen: Screen) {
        self.next_screen = next_screen;
    }

    pub fn get_position(&self) -> i32 {
        self.position
    }
    pub fn set_position(&mut self, position: i32) {
        self.position = position
    }

    // pub fn get_words(&self) -> &Vec<String> {
    //     &self.words
    // }
    // pub fn set_words(&mut self, words: &[String; 5]) {
    //     self.words = words.to_vec();
    // }

    pub fn get_paragraph(&self) -> &String {
        &self.paragraph
    }
    pub fn set_paragraph(&mut self, new_paragraph: String) {
        self.paragraph = new_paragraph;
    }

    pub fn get_timer(&self) -> &String {
        &self.timer
    }
    pub fn set_timer(&mut self, new_timer: String) {
        self.timer = new_timer;
    }

    pub fn get_duration(&self) -> u64 {
        self.duration
    }
    pub fn set_duration(&mut self, duration: u64) {
        self.duration = duration;
    }
}
