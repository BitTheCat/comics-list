use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginationRequest {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub query: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub page: i64,
    pub page_size: i64,
    pub total_items: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize)]
pub struct PaginationResponse<T> {
    pub items: Vec<T>,
    pub pagination: PaginationMeta,
}
