#![allow(non_snake_case)]
use super::storage_type_list::get_storage_types;
use crate::config::BACKEND_URI;
use crate::error::Result;
use common::StorageTypeCreated;
use dioxus::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct AcstorFeatureCreated {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

pub async fn create_acstor_feature(
    name: String,
    description: Option<String>,
    storage_type_ids: Vec<i32>,
) -> Result<AcstorFeatureCreated> {
    let client = Client::new();
    let created = client
        .post(format!("{BACKEND_URI}/api/acstor_features"))
        .json(&json!({
            "name": name,
            "description": description,
            "storage_types": storage_type_ids
        }))
        .send()
        .await?
        .json::<AcstorFeatureCreated>()
        .await?;
    Ok(created)
}

#[component]
pub fn AcstorFeatureCreate() -> Element {
    let mut future: Resource<Result<Vec<StorageTypeCreated>>> =
        use_resource(|| get_storage_types());

    rsx!(
        h1 { "AcstorFeatureCreate" }
    )
}
