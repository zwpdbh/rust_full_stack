#![allow(unused)]
#![allow(non_snake_case)]
use super::storage_type_list::get_storage_types;
use crate::components::MyFormDiv;
use crate::error::Result;
use crate::{components::FormButton, config::BACKEND_URI};
use common::StorageTypeCreated;
use dioxus::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

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

    let create_acstor_feature_action = move |event: MouseEvent| {
        event.prevent_default();
        info!("create_acstor_feature_action ->> ");
    };

    let selected_storage_type_ids = use_signal(|| Vec::<i32>::new());

    rsx!(
        h1 { "AcstorFeatureCreate" }

        MyFormDiv {
            h1 { class: "text-2xl font-bold mb-6 text-center", "Create a New ACStor Feature" }
            form { class: "space-y-4",
                match &*future.read_unchecked() {
                    Some(Ok(storage_types)) => rsx! {},
                    Some(Err(e)) => rsx! {
                        button { onclick: move |_| future.restart(), "Reload" }
                        div { class: "error",
                            h3 { "Error loadding StorageTypes" }
                            p { "An error occurred: {e}" }
                        }
                    },
                    None => {
                        rsx! {
                            div { "Loading storage types..." }
                        }
                    }
                }
                FormButton { label: "Submit", onclick: create_acstor_feature_action }
            }
        }
    )
}
