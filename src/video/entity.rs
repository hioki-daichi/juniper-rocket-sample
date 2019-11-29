use chrono::NaiveDateTime;

#[derive(Debug, Queryable)]
pub struct Video {
    pub id: i64,
    pub src: String,
    pub timestamp: NaiveDateTime,
}
