use dioxus::{events::MouseEvent, prelude::*};

#[component]
pub fn FormButton(label: String, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}
