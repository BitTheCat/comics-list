use serde::Deserialize;

use crate::models::pagination::PaginationRequest;

#[derive(Debug, Deserialize)]
pub struct EntryPayload {
    pub id: Option<i64>,
    pub series_id: i64,
    pub issue_number: String,
    pub title: Option<String>,
    pub pub_year: Option<i32>,
    pub pub_month: Option<i32>,
    pub cover_path: Option<String>,
    pub tags: Option<Vec<String>>,
    pub condition: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ListEntriesRequest {
    #[serde(flatten)]
    pub pagination: PaginationRequest,

    pub series_id: i64,
}
