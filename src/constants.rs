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

#[derive(Debug, Default, PartialEq)]
pub enum Screen {
    Main,
    #[default]
    Menu,
    CountDown,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Init,
    Empty,
    Exit,
    CharInput(char),
    ChangeScene(Screen),
}
