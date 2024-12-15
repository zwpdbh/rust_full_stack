#![allow(unused)]
mod form_button;
use dioxus::prelude::*;
pub use form_button::*;

mod form_input;
pub use form_input::*;

mod form_textarea;
pub use form_textarea::*;

// #[derive(PartialEq, Clone, Props)]
// pub struct FormFieldProps {
//     label: String,
//     value: String,
//     onchange: EventHandler<FormEvent>,
// }

#[component]
pub fn FormField(label: String, value: String, onchange: EventHandler<FormEvent>) -> Element {
    rsx! {
        div {
            label { "{label}" }
            div {
                input { value: "{value}", oninput: move |e| onchange.call(e) }
            }
        }
    }
}
