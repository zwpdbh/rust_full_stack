#![allow(non_snake_case)]
use crate::routes::Route;
use dioxus::prelude::*;

pub use post_create::PostCreate;
pub use post_detail::PostRead;
pub use post_list::PostList;

mod post_create;
mod post_detail;
mod post_list;

#[component]
pub fn Blog() -> Element {
    rsx!(
        div { class: "colums",
            div { class: "column is-one-fifth", BlogMenu {} }
            div { class: "column", Outlet::<Route> {} }
        }
    )
}

#[component]
fn BlogMenu() -> Element {
    rsx!(
        aside { class: "menu",
            p { class: "menu-label", "Posts" }
            ul { class: "menu-list",
                li {
                    Link { to: Route::PostList {}, "List" }
                }
                li {
                    Link { to: Route::PostCreate {}, "Create" }
                }
            }
        }
    )
}
