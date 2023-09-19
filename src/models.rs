use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateLinkModel {
    pub url: String,
}
