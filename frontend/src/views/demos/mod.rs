#![allow(non_snake_case)]
use crate::{
    components::{css_config, MenuItem, MenuSection},
    routes::Route,
};
use dioxus::prelude::*;

mod async_with_coroutines;
mod async_with_resource;
mod async_with_spawn;
mod context;
mod dynamic_rendering;
mod event_handler;
mod hooks;
mod llm;
mod prop;
mod rsx_basic;
mod user_input;

pub use async_with_coroutines::DemoCoroutines;
pub use async_with_resource::DemoResource;
pub use async_with_spawn::DemoSpawn;
pub use context::DemoContext;
pub use dynamic_rendering::DemoDynamicRendering;
pub use event_handler::DemoEventHandler;
pub use hooks::DemoHooks;
pub use llm::DemoLLM;
pub use prop::DemoProp;
pub use rsx_basic::RsxBasic;
pub use user_input::UserInput;

/// Place holder for Demo section
#[component]
pub fn Demo() -> Element {
    rsx!(
        div { class: css_config::SECTION_DIV,
            div {
                div { class: css_config::SECTION_MENU, DemoMenu {} }
            }
            div { class: css_config::SECTION_CONTENT, Outlet::<Route> {} }
        }
    )
}

#[component]
pub fn DemoMenuDefault() -> Element {
    rsx!()
}

/// This is the sidebar menu to show different demos for demo section
#[component]
fn DemoMenu() -> Element {
    rsx!(
        document::Link { rel: "stylesheet", href: css_config::SIDEBAR_CSS }
        aside {
            MenuSection {
                title: "General",
                items: vec![
                    ("RsxBasic", Some(Route::RsxBasic {})),
                    ("Prop", Some(Route::DemoProp {})),
                    ("Event Handler", Some(Route::DemoEventHandler {})),
                    ("Hooks", Some(Route::DemoHooks {})),
                    ("User Input", Some(Route::UserInput {})),
                    ("Context", Some(Route::DemoContext {})),
                    ("Dynamic Rendering", Some(Route::DemoDynamicRendering {})),
                    ("Async with Resource", Some(Route::DemoResource {})),
                    ("Async with Coroutines", Some(Route::DemoCoroutines {})),
                    ("Async with Spawn", Some(Route::DemoSpawn {})),
                ],
            }
            MenuSection {
                title: "LLM service",
                items: vec![("LLM service", Some(Route::DemoLLM {}))],
            }
            MenuSection {
                title: "ACStor CRUD",
                items: vec![
                    ("Team Settings", None),
                    ("Manage Your Team", None),
                    ("Invitations", None),
                    ("Cloud Storage Environment Settings", None),
                    ("Authentication", None),
                ],
            }

            // Submenu for "Manage Your Team"
            p { class: "text-gray-500 text-xs uppercase font-semibold mt-2 mb-1",
                "Manage Your Team"
            }
            ul { class: "ml-4 space-y-1",
                MenuItem { label: "Members" }
                MenuItem { label: "Plugins" }
                MenuItem { label: "Add a member" }
            }
        }
    )
}

#[component]
fn MyCard(children: Element) -> Element {
    rsx!(
        div {
            div {
                // Notice the children is placed inside "{{}}"
                div { {children} }
            }
        }
    )
}
