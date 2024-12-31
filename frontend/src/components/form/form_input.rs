use dioxus::prelude::*;

#[component]
pub fn FormInputText(label: String, value: String, onchange: EventHandler<FormEvent>) -> Element {
    rsx! {
        div {
            label { "{label}" }
            div {
                input {
                    value: "{value}",
                    oninput: move |e| onchange.call(e),
                    class: "border border-gray-300 focus:outline-none",
                }
            }
        }
    }
}
