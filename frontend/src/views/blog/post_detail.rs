#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn PostRead(id: String) -> Element {
    rsx!(
        h1 { "page for see a post with id: {id}" }
    )
}
