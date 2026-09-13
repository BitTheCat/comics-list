use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateSeriesPayload {
    pub title: String,
    pub description: Option<String>,
    pub publisher_id: Option<i64>,
    pub start_year: Option<i32>,
    pub end_year: Option<i32>,
    pub notes: Option<String>,
}
