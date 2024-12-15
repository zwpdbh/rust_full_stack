#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::*;
use crate::views::blog::*;
use crate::views::demos::*;

#[rustfmt::skip]
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    // All routes under the NavBar layout will be rendered inside of the NavBar Outlet
    #[layout(NavBar)]
        #[route("/")]
        Home {},

        // For blog section
        #[nest("/blog")]
            #[layout(Blog)]
                #[route("/")]
                PostList {},

                #[route("/post/:id")]
                PostRead {id: String},

                #[route("/post")]
                PostCreate {},
            #[end_layout]
        #[end_nest]

        // For demo section 
        #[nest("/demo")]
            #[layout(Demo)]
                #[route("/")]
                DemoMenuDefault {},

                #[route("/rsxbasic")]
                RsxBasic {},

                #[route("/prop")]
                DemoProp {},

                #[route("/event_handler")]
                DemoEventHandler {},

                #[route("/hoooks")]
                DemoHooks {},

                #[route("/userinput")]
                UserInput {},

                #[route("/context")]
                DemoContext {},

                #[route("/dynamicrendering")]
                DemoDynamicRendering {},

                #[route("/demo_resource")]
                DemoResource {},

                #[route("/demo_coroutines")]
                DemoCoroutines {},

                #[route("/demo_spawn")]
                DemoSpawn {},

                #[route("/llm")]
                DemoLLM {},
            #[end_layout]
        #[end_nest]
    #[end_layout]

    // This will redirect user to /blog or /blog/post/:id 
    #[nest("/myblog")]
        #[redirect("/", || Route::PostList {})]
        #[redirect("/:id", |id: String| Route::PostRead { id })]
    #[end_nest]

    // #[route("/acstor")]
    // Acstor {},
    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
