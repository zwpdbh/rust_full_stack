use dioxus::prelude::*;

// #[derive(PartialEq, Props, Clone)]
// pub struct FormTextareaProps {
//     oninput: EventHandler<FormData>,

//     #[props(!optional)]
//     rows: Option<u8>,
//     #[props(!optional)]
//     placeholder: Option<String>,
// }

// pub fn FormTextarea_Lg<'a>(cx: Scope<'a, FormTextareaProps<'a>>) -> Element {
//     let rows = cx.props.rows.unwrap_or(1);
//     let ph = cx.props.placeholder.clone().unwrap_or_default();
//     cx.render(rsx! {
//         fieldset { class: "form-group",
//             textarea {
//                 class: "form-control form-control-lg",
//                 oninput: move |evt| cx.props.oninput.call(evt.data.as_ref().clone()),
//                 placeholder: "{ph}",
//                 rows: "{rows}"
//             }
//         }
//     })
// }

#[component]
pub fn FormTextarea_Lg(rows: Option<u8>, placeholder: Option<String>) -> Element {
    let rows = rows.unwrap_or(1);
    let placeholder = placeholder.unwrap_or("".to_string());

    rsx!(
        fieldset { class: "form-group",
            textarea {
                class: "form-control form-control-lg",
                placeholder: "{placeholder}",
                rows: "{rows}"
            }
        }
    )
}
