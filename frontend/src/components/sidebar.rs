#![allow(non_snake_case)]
use crate::routes::Route;
use dioxus::prelude::*;

pub mod css_config {
    use dioxus::prelude::*;

    pub const SIDEBAR_CSS: Asset = asset!("/assets/styling/sidebar.css");
    pub const SECTION_CONTENT: &str = "flex-1 p-4 overflow-auto";
    pub const SECTION_MENU: &str = "w-64 h-screen bg-gray-100 p-4 overflow-y-auto";
    pub const SECTION_DIV: &str = "flex h-screen";
}

#[component]
pub fn MenuSection(title: &'static str, items: Vec<(&'static str, Option<Route>)>) -> Element {
    rsx!(
        p { class: "text-gray-500 text-xs uppercase font-semibold mb-2", "{title}" }

        ul { class: "space-y-1 mb-4",
            for (label , route) in items {
                MenuItem { label, route }
            }
        }
    )
}

#[component]
pub fn MenuItem(label: &'static str, route: Option<Route>) -> Element {
    let class = "block hover:bg-gray-200 px-2 py-1 rounded";
    match route {
        Some(r) => rsx!(
            li {
                Link { to: r, class, "{label}" }
            }
        ),
        None => rsx!(
            li {
                a { class, "{label}" }
            }
        ),
    }
}
