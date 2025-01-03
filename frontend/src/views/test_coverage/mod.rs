#![allow(non_snake_case)]
use crate::components::{css_config, MenuSection};
use crate::routes::Route;
use dioxus::prelude::*;

mod acstor_feature_create;
mod acstor_feature_list;
mod storage_type_create;
mod storage_type_list;

pub use acstor_feature_create::AcstorFeatureCreate;
pub use acstor_feature_list::AcstorFeatureList;
pub use storage_type_create::StorageTypeCreate;
pub use storage_type_list::StorageTypeList;

#[component]
pub fn TestCoverage() -> Element {
    rsx! {
        div { class: css_config::SECTION_DIV,
            div { class: css_config::SECTION_MENU, TestCoverageMenu {} }
            div { class: css_config::SECTION_CONTENT, Outlet::<Route> {} }
        }
    }
}

#[component]
fn TestCoverageMenu() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: css_config::SIDEBAR_CSS }
        aside {
            MenuSection {
                title: "Storage Types",
                items: vec![
                    ("List", Some(Route::StorageTypeList {})),
                    ("Create", Some(Route::StorageTypeCreate {})),
                ],
            }
            MenuSection {
                title: "ACStor Features",
                items: vec![
                    ("List", Some(Route::AcstorFeatureList {})),
                    ("Create", Some(Route::AcstorFeatureCreate {})),
                ],
            }
        }
    }
}
