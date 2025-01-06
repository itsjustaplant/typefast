#[derive(Debug, Default, Clone, PartialEq)]
pub struct Record {
    pub id: i32,
    pub wpm: i64,
    pub cpm: i64,
    pub date: String,
}
