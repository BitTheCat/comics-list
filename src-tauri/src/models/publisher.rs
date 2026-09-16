use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePublisherPayload {
    pub name: String,
}
