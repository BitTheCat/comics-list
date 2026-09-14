use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateEntryPayload {
    pub id: i64,
    pub series_id: i64,
    pub issue_number: String,
    pub title: Option<String>,
    pub pub_year: Option<i32>,
    pub pub_month: Option<i32>,
    pub cover_path: Option<String>,
    pub tags: Option<String>,
    pub condition: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
