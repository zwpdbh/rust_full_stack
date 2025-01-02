#![allow(non_snake_case)]

use config::*;
use dioxus::prelude::*;
use routes::Route;

mod components;
mod config;
mod error;
mod routes;
mod views;

fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

fn main() {
    // Init logger
    launch(App);
}
