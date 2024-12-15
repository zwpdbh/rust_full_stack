use dioxus::{events::MouseEvent, prelude::*};

#[component]
pub fn FormButton(label: String, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button {
            class: "btn btn-lg btn-primary pull-xs-right",
            r#type: "button",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}
