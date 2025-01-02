use dioxus::prelude::*;
use tracing::info;

#[component]
pub fn FormMultipleSelect(
    label: String,
    options: Vec<String>,
    selected: Vec<String>,
    onchange: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        label { "{label}" }
        ul {
            for each_option in options {
                li {
                    label { class: "checkbox",
                        "{each_option}"
                        input {
                            r#type: "checkbox",
                            onchange: move |event| { info!("{event:?}") },
                        }
                    }
                }
            }
        }
    }
}
