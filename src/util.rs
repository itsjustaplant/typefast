pub fn calculate_word_speed(word_count: u64, duration: u64) -> i64 {
    if duration == 0 {
        return 0_i64;
    }
    let duration = duration as f64;
    let word_count = word_count as f64;
    (word_count / (duration / 60.0)).round() as i64
}
pub fn calculate_char_speed(char_count: u64, duration: u64) -> i64 {
    if duration == 0 {
        return 0_i64;
    }
    let duration = duration as f64;
    let char_count = char_count as f64;
    (char_count / (duration / 60.0)).round() as i64
}
