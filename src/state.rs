use crate::constants::Page;
use crate::record::Record;

#[derive(Debug, Default)]
pub struct State {
    pub is_running: bool,
    pub page: Page,
    pub next_page: Page,
    pub error: String,
    pub position: i32,
    pub paragraph: String,
    pub timer: u64,
    pub char_count: u64,
    pub word_count: u64,
    pub char_speed: i64,
    pub word_speed: i64,
    pub records: Vec<Record>,
    pub menu_index: i32,
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

    pub fn get_page(&self) -> &Page {
        &self.page
    }
    pub fn set_page(&mut self, page: Page) {
        self.page = page;
    }

    pub fn get_next_page(&self) -> &Page {
        &self.next_page
    }
    pub fn set_next_page(&mut self, next_page: Page) {
        self.next_page = next_page;
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

    pub fn get_char_speed(&self) -> i64 {
        self.char_speed
    }
    pub fn set_char_speed(&mut self, char_speed: i64) {
        self.char_speed = char_speed;
    }

    pub fn get_word_speed(&self) -> i64 {
        self.word_speed
    }
    pub fn set_word_speed(&mut self, word_speed: i64) {
        self.word_speed = word_speed;
    }

    pub fn get_records(&self) -> &Vec<Record> {
        &self.records
    }
    pub fn set_records(&mut self, records: Vec<Record>) {
        self.records = records;
    }

    pub fn get_menu_index(&self) -> i32 {
        self.menu_index
    }
    pub fn set_menu_index(&mut self, menu_index: i32) {
        self.menu_index = menu_index;
    }
}
