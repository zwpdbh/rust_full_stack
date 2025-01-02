#![allow(non_snake_case)]
use crate::config::BACKEND_URI;
use crate::error::Result;
use crate::routes::Route;
use common::StorageTypeCreated;
use dioxus::prelude::*;
use reqwest::Client;
use tracing::info;

#[component]
pub fn StorageTypeList() -> Element {
    rsx! {
        h1 { "StorageTypeList" }
        RenderListStorageTypesResult {}
    }
}

pub async fn get_storage_types() -> Result<Vec<StorageTypeCreated>> {
    let client = Client::new();
    let url = format!("{BACKEND_URI}/api{}", Route::StorageTypeList {});

    info!("->> {url}");
    let storage_types = client
        .get(url)
        .send()
        .await?
        .json::<Vec<StorageTypeCreated>>()
        .await?;

    Ok(storage_types)
}

#[component]
fn RenderListStorageTypesResult() -> Element {
    let mut future: Resource<Result<Vec<StorageTypeCreated>>> =
        use_resource(|| get_storage_types());

    rsx! {
        match &*future.read_unchecked() {
            Some(Ok(storage_types)) => rsx! {
                button { onclick: move |_| future.restart(), "Reload" }
                table {
                    tr {
                        th { "Id" }
                        th { "Name" }
                        th { "Description" }
                    }
                    for storage_type in storage_types {
                        tr {
                            td { "{storage_type.id}" }
                            td { "{storage_type.name}" }
                            td { "{storage_type.description:?}" }
                        }
                    }
                }
            },
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
    }
}
