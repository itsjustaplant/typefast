pub use std::fmt;

pub const APP_PATH: &str = "typefast";
pub const DB_NAME: &str = "typefast.db";
pub const TEST_APP_PATH: &str = "__test__";
pub const TEST_DB_NAME: &str = "mock.db";
pub const GAME_DURATION: u64 = 60;
pub const COUNTDOWN_DURATION: u64 = 3;
pub const TEST_WORDS: [&str; 21] = [
    "plant",
    "planta",
    "plantable",
    "plantad",
    "Plantae",
    "some",
    "words",
    "abc",
    "hmmm",
    "why",
    "do",
    "do",
    "do",
    "do",
    "do",
    "do",
    "do",
    "do",
    "even",
    "type",
    "more",
];
pub const WORD_LIST: &str = include_str!("../assets/word_list.txt");
pub const MENU_ITEMS: [&str; 2] = ["Start", "Records"];

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum Page {
    Game,
    #[default]
    Menu,
    CountDown,
    Records,
    GameResult,
}

impl fmt::Display for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Page::Game => write!(f, "Game"),
            Page::Menu => write!(f, "Menu"),
            Page::CountDown => write!(f, "CountDown"),
            Page::Records => write!(f, "Records"),
            Page::GameResult => write!(f, "GameResult"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Init,
    Empty,
    Exit,
    CharInput(char),
    ChangePage(Page),
    GetRecords,
    PostRecord,
    MenuAction,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page() {
        assert_eq!(Page::Game.to_string(), "Game");
        assert_eq!(Page::Menu.to_string(), "Menu");
        assert_eq!(Page::CountDown.to_string(), "CountDown");
        assert_eq!(Page::Records.to_string(), "Records");
        assert_eq!(Page::GameResult.to_string(), "GameResult");
    }

    #[test]
    fn test_page_default() {
        assert_eq!(Page::default(), Page::Menu);
    }

    #[test]
    fn test_action() {
        assert_eq!(Action::Init, Action::Init);
        assert_eq!(Action::Empty, Action::Empty);
        assert_eq!(Action::Exit, Action::Exit);
        assert_eq!(Action::CharInput('a'), Action::CharInput('a'));
        assert_eq!(
            Action::ChangePage(Page::Game),
            Action::ChangePage(Page::Game)
        );
        assert_eq!(Action::GetRecords, Action::GetRecords);
        assert_eq!(Action::PostRecord, Action::PostRecord);
        assert_eq!(Action::MenuAction, Action::MenuAction);
    }

    #[test]
    fn test_constants() {
        assert_eq!(APP_PATH, "typefast");
        assert_eq!(DB_NAME, "typefast.db");
        assert_eq!(GAME_DURATION, 60);
        assert_eq!(COUNTDOWN_DURATION, 3);
        assert_eq!(TEST_WORDS.len(), 21);
        assert_eq!(MENU_ITEMS, ["Start", "Records"]);
    }
}
