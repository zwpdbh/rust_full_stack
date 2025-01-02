#![allow(unused)]
mod form_button;
use dioxus::prelude::*;
pub use form_button::*;

mod form_input;
pub use form_input::*;

mod form_select;
pub use form_select::*;

#[component]
pub fn MyFormDiv(children: Element) -> Element {
    rsx!(
        div { class: "max-w-md mx-auto mt-8 p-6 bg-white rounded-lg shadow-md", {children} }
    )
}
