pub use std::fmt;

pub const APP_PATH: &str = "typefast";
pub const DB_NAME: &str = "typefast.db";
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
