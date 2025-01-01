#![allow(unused)]
#![allow(non_snake_case)]
use crate::error::Result;
use common::test_coverage;
use dioxus::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

pub async fn create_acstor_feature_v1(
    name: String,
    description: Option<String>,
    storage_type_ids: Vec<i32>,
) -> Result<test_coverage::CreatedAcstorFeature> {
    let client = Client::new();
    let created = client
        .post("http://localhost::5150/api/acstor_features")
        .json(&json!({
            "name": name,
            "description": description,
            "storage_types": storage_type_ids
        }))
        .send()
        .await?
        .json::<test_coverage::CreatedAcstorFeature>()
        .await?;
    Ok(created)
}

pub async fn create_acstor_feature(
    params: test_coverage::CreateAcstorFeatureFormParams,
) -> Result<test_coverage::CreatedAcstorFeature> {
    let client = Client::new();
    let created = client
        .post("http://localhost::5150/api/acstor_features")
        .json(&params)
        .send()
        .await?
        .json::<test_coverage::CreatedAcstorFeature>()
        .await?;
    Ok(created)
}

#[component]
pub fn AcstorFeatureCreate() -> Element {
    rsx!(
        h1 { "AcstorFeatureCreate" }
    )
}
