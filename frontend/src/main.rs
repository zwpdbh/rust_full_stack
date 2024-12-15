#![allow(non_snake_case)]

use dioxus::prelude::*;
use routes::Route;

mod components;
mod error;
mod routes;
mod views;

const FAVICON: Asset = asset!("./assets/favicon.ico");
const MAIN_CSS: Asset = asset!("./assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("./assets/tailwind.css");

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
