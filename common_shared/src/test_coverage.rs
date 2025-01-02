use std::fmt;

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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct StorageTypeCreated {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

impl fmt::Display for StorageTypeCreated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = format!("name: {}", self.name);
        write!(f, "{}", output)
    }
}
