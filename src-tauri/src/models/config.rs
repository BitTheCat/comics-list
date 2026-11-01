use serde::Deserialize;

/// Partial update of the app config. Missing fields are left untouched;
/// an empty string clears an optional value (e.g. `db_path`).
#[derive(Debug, Default, Deserialize)]
pub struct UpdateConfigPayload {
    pub db_path: Option<String>,
    pub covers_dir: Option<String>,
    pub theme: Option<String>,
    pub language: Option<String>,
}
