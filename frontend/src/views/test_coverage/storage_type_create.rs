#![allow(non_snake_case)]
use std::fmt;

use crate::components::{FormButton, FormInputText, MyFormDiv};
use crate::error::Result;
use dioxus::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

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

pub async fn create_storage_type(
    name: String,
    description: Option<String>,
) -> Result<StorageTypeCreated> {
    let client = Client::new();
    let post_created = client
        .post("http://localhost:5150/api/storage_types")
        .json(&json!({
            "name": name,
            "description": description
        }))
        .send()
        .await?
        .json::<StorageTypeCreated>()
        .await?;

    Ok(post_created)
}

#[derive(Debug, PartialEq, Clone)]
pub enum CreateStorageTypeResult {
    NotStarted,
    InProgress,
    Finished(StorageTypeCreated),
    Error(String),
}

#[component]
pub fn StorageTypeCreate() -> Element {
    let mut storage_type_name = use_signal(|| "".to_string());
    let mut storage_type_description = use_signal(|| None);
    let mut create_storage_type_result = use_signal(|| CreateStorageTypeResult::NotStarted);
    let create_storage_type_action = move |event: MouseEvent| {
        event.prevent_default();
        info!(
            "create_storage_type_action ->> - name: {}, description: {:?}",
            storage_type_name(),
            storage_type_description()
        );

        let _ = spawn(async move {
            create_storage_type_result.set(CreateStorageTypeResult::InProgress);

            let post = create_storage_type(storage_type_name(), storage_type_description()).await;
            match post {
                Ok(post) => create_storage_type_result.set(CreateStorageTypeResult::Finished(post)),
                Err(e) => {
                    create_storage_type_result.set(CreateStorageTypeResult::Error(e.to_string()))
                }
            }
        });
    };

    rsx!(
        h1 { "StorageTypeCreate" }


        MyFormDiv {
            h1 { class: "text-2xl font-bold mb-6 text-center", "Create a New StorageType" }

            form { class: "space-y-4",
                FormInputText {
                    label: "Feature Name",
                    value: storage_type_name(),
                    onchange: move |e: FormEvent| storage_type_name.set(e.value().clone()),
                }

                FormInputText {
                    label: "Feature Description",
                    value: storage_type_description().unwrap_or("".to_string()),
                    onchange: move |e: FormEvent| storage_type_description.set(Some(e.value().clone())),
                }

                FormButton { label: "Submit", onclick: create_storage_type_action }
            }

            RenderCreateStorageTypeResult { create_storage_type_result: create_storage_type_result() }
        }
    )
}

#[component]
fn RenderCreateStorageTypeResult(create_storage_type_result: CreateStorageTypeResult) -> Element {
    rsx!(
        div { class: "mt-4",
            h2 { "Created StorageType" }
            match create_storage_type_result {
                CreateStorageTypeResult::NotStarted => rsx! {
                    p { "No StorageType has been created yet." }
                },
                CreateStorageTypeResult::InProgress => rsx! {
                    p { "Creating StorageType..." }
                },
                CreateStorageTypeResult::Finished(storage_type_created) => {
                    rsx! {
                        div {
                            h3 { "StorageType Created Successfully" }
                            p { "{storage_type_created}" }
                        }
                    }
                }
                CreateStorageTypeResult::Error(e) => rsx! {
                    div { class: "error",
                        h3 { "Error Creating StorageType" }
                        p { "An error occurred: {e}" }
                    }
                },
            }
        }
    )
}
