use chrono::Local;

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

pub fn get_current_datetime() -> String {
    let now = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_word_speed() {
        assert_eq!(calculate_word_speed(120, 60), 120);
        assert_eq!(calculate_word_speed(0, 60), 0);
        assert_eq!(calculate_word_speed(120, 0), 0);
        assert_eq!(calculate_word_speed(60, 120), 30);
    }

    #[test]
    fn test_calculate_char_speed() {
        assert_eq!(calculate_char_speed(600, 60), 600);
        assert_eq!(calculate_char_speed(0, 60), 0);
        assert_eq!(calculate_char_speed(600, 0), 0);
        assert_eq!(calculate_char_speed(300, 120), 150);
    }

    #[test]
    fn test_get_current_datetime() {
        let datetime = get_current_datetime();
        assert!(datetime.len() > 0);
    }
}
