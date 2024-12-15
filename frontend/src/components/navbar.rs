#![allow(non_snake_case)]
use crate::routes::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx!(
        header { class: "bg-white",
            nav { class: "flex items-center justify-between w-[92%]  mx-auto",
                div { class: "w-16",
                    p { class: "uppercase font-semibold", "acstor" }
                }
                div {
                    ul { class: "flex items-center gap-[4vw]",
                        NavBarItem { label: "Home", route: Some(Route::Home {}) }
                        NavBarItem {
                            label: "Demos",
                            route: Some(Route::DemoMenuDefault {}),
                        }
                        NavBarItem {
                            label: "Blog List",
                            route: Some(Route::PostList {}),
                        }
                        NavBarItem { label: "More", route: None }
                    }
                }
                div {
                    button { class: "hover:text-gray-500", "Sign In " }
                }
            }
        }

        // The Outlet component will render child routes (In this case just the Home component) inside the Outlet component
        Outlet::<Route> {}
    )
}

#[component]
fn NavBarItem(label: &'static str, route: Option<Route>) -> Element {
    let class = "hover:text-gray-500";
    match route {
        Some(r) => rsx!(
            li {
                Link { to: r, class, "{label}" }
            }
        ),
        None => rsx!(
            li {
                a { class, "{label}" }
            }
        ),
    }
}
