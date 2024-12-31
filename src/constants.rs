pub const APP_PATH: &str = "typefast";
pub const DB_NAME: &str = "typefast.db";
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
    #[default]
    Main,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Init,
    Empty,
    Exit,
    CharInput(char),
}
