#![allow(non_snake_case)]
use dioxus::prelude::*;

mod navbar;
pub use navbar::NavBar;

mod form;
pub use form::*;

#[component]
pub fn Home() -> Element {
    rsx!(
        h1 { "This is home page" }
    )
}

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
