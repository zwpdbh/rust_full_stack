use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateAcstorFeatureFormParams {
    pub name: String,
    pub description: Option<String>,
    pub storage_type_ids: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CreatedAcstorFeature {
    pub name: String,
    pub description: Option<String>,
    pub storage_type_ids: Vec<i32>,
}
