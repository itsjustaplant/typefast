use crate::constants::Screen;

#[derive(Debug, Default)]
pub struct State {
    pub is_running: bool,
    pub screen: Screen,
    pub next_screen: Screen,
    pub error: String,
    pub position: i32,
    pub paragraph: String,
    pub timer: u64,
    pub char_count: u64,
    pub word_count: u64,
    pub char_speed: f64,
    pub word_speed: f64,
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

    pub fn get_paragraph(&self) -> &String {
        &self.paragraph
    }
    pub fn set_paragraph(&mut self, new_paragraph: String) {
        self.paragraph = new_paragraph;
    }

    pub fn get_timer(&self) -> u64 {
        self.timer
    }
    pub fn set_timer(&mut self, new_timer: u64) {
        self.timer = new_timer;
    }

    pub fn get_word_count(&self) -> u64 {
        self.word_count
    }
    pub fn set_word_count(&mut self, word_count: u64) {
        self.word_count = word_count;
    }

    pub fn get_char_count(&self) -> u64 {
        self.char_count
    }
    pub fn set_char_count(&mut self, char_count: u64) {
        self.char_count = char_count;
    }

    pub fn get_char_speed(&self) -> f64 {
        self.char_speed
    }
    pub fn set_char_speed(&mut self, char_speed: f64) {
        self.char_speed = char_speed;
    }

    pub fn get_word_speed(&self) -> f64 {
        self.word_speed
    }
    pub fn set_word_speed(&mut self, word_speed: f64) {
        self.word_speed = word_speed;
    }
}
