pub const APP_PATH: &str = "typefast";
pub const DB_NAME: &str = "typefast.db";

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
}
