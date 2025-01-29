use chrono::Local;

use crate::constants::Page;
use crate::record::Record;

#[derive(Debug, Default)]
pub struct State {
    pub is_running: bool,
    pub page: Page,
    pub error: String,
    pub position: i32,
    pub paragraph: String,
    pub timer: i64,
    pub char_count: u64,
    pub word_count: u64,
    pub records: Vec<Record>,
    pub menu_index: i32,
    pub reference_timestamp: i64,
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

    pub fn get_timer(&self) -> i64 {
        self.timer
    }
    pub fn set_timer(&mut self, new_timer: i64) {
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

    pub fn reset_stats(&mut self) {
        self.set_char_count(0);
        self.set_position(0);
        self.set_word_count(0);
    }

    pub fn get_reference_timestamp(&self) -> i64 {
        self.reference_timestamp
    }
    pub fn set_reference_timestamp(&mut self, timestamp: Option<i64>) {
        self.reference_timestamp = timestamp.unwrap_or(Local::now().timestamp_millis());
    }

    pub fn get_elapsed_time(&self) -> i64 {
        if self.reference_timestamp == 0 {
            0
        } else {
            (Local::now().timestamp_millis() - self.get_reference_timestamp()) / 1000
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_initialization() {
        let state = State::new();
        assert_eq!(state.is_running, false);
        assert_eq!(state.page, Page::default());
        assert_eq!(state.error, "");
        assert_eq!(state.position, 0);
        assert_eq!(state.paragraph, "");
        assert_eq!(state.timer, 0);
        assert_eq!(state.char_count, 0);
        assert_eq!(state.word_count, 0);
        assert!(state.records.is_empty());
        assert_eq!(state.menu_index, 0);
        assert_eq!(state.reference_timestamp, 0);
    }

    #[test]
    fn test_set_get_is_running() {
        let mut state = State::new();
        state.set_is_running(true);
        assert_eq!(state.get_is_running(), true);
    }

    #[test]
    fn test_set_get_error() {
        let mut state = State::new();
        let error_message = String::from("An error occurred");
        state.set_error(error_message.clone());
        assert_eq!(state.get_error(), &error_message);
    }

    #[test]
    fn test_set_get_page() {
        let mut state = State::new();
        let page = Page::default();
        state.set_page(page.clone());
        assert_eq!(state.get_page(), &page);
    }

    #[test]
    fn test_set_get_position() {
        let mut state = State::new();
        state.set_position(10);
        assert_eq!(state.get_position(), 10);
    }

    #[test]
    fn test_set_get_paragraph() {
        let mut state = State::new();
        let paragraph = String::from("This is a test paragraph.");
        state.set_paragraph(paragraph.clone());
        assert_eq!(state.get_paragraph(), &paragraph);
    }

    #[test]
    fn test_set_get_timer() {
        let mut state = State::new();
        state.set_timer(100);
        assert_eq!(state.get_timer(), 100);
    }

    #[test]
    fn test_set_get_word_count() {
        let mut state = State::new();
        state.set_word_count(50);
        assert_eq!(state.get_word_count(), 50);
    }

    #[test]
    fn test_set_get_char_count() {
        let mut state = State::new();
        state.set_char_count(200);
        assert_eq!(state.get_char_count(), 200);
    }

    #[test]
    fn test_set_get_reference_timestamp() {
        let mut state = State::new();
        let timestamp = 1627846261;
        state.set_reference_timestamp(Some(timestamp));
        assert_eq!(state.get_reference_timestamp(), timestamp);
    }

    #[test]
    fn test_set_reference_timestamp_none() {
        let mut state = State::new();
        state.set_reference_timestamp(None);
        assert!(state.get_reference_timestamp() > 0);
    }
}
