#![allow(non_snake_case)]
use std::fmt;

use crate::components::{FormButton, FormInputText, MyFormDiv};
use crate::config::BACKEND_URI;
use crate::error::Result;
use common::StorageTypeCreated;
use dioxus::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

pub async fn create_storage_type(
    name: String,
    description: Option<String>,
) -> Result<StorageTypeCreated> {
    let client = Client::new();
    let created = client
        .post(format!("{BACKEND_URI}/api/storage_types"))
        .json(&json!({
            "name": name,
            "description": description
        }))
        .send()
        .await?
        .json::<StorageTypeCreated>()
        .await?;

    Ok(created)
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
                    label: "Storage Type Name",
                    value: storage_type_name(),
                    onchange: move |e: FormEvent| storage_type_name.set(e.value().clone()),
                }

                FormInputText {
                    label: "Storage Type Description",
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
