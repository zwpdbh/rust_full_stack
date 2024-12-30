#![allow(non_snake_case)]
use crate::components::css_config;
use crate::routes::Route;
use dioxus::prelude::*;

pub use post_create::PostCreate;
pub use post_detail::PostRead;
pub use post_list::PostList;

mod post_create;
mod post_detail;
mod post_list;
use crate::components::MenuSection;

#[component]
pub fn Blog() -> Element {
    rsx!(
        div { class: css_config::SECTION_DIV,
            div { class: css_config::SECTION_MENU, BlogMenu {} }
            div { class: css_config::SECTION_CONTENT, Outlet::<Route> {} }
        }
    )
}

#[component]
fn BlogMenu() -> Element {
    rsx!(
        document::Link { rel: "stylesheet", href: css_config::SIDEBAR_CSS }
        aside {
            MenuSection {
                title: "Posts",
                items: vec![("List", Some(Route::PostList {})), ("Create", Some(Route::PostCreate {}))],
            }
        }
    )
}
