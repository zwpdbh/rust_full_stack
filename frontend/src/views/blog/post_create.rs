#![allow(unused)]
#![allow(non_snake_case)]
use crate::components::{FormButton, FormInputText, MyFormDiv};
use crate::error::Result;
use dioxus::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PostCreated {
    pub id: i32,
    pub title: String,
    pub content: String,
}

pub async fn create_post(title: &str, content: &str) -> Result<PostCreated> {
    let client = Client::new();
    let post_created = client
        .post("http://localhost:5150/api/posts")
        .json(&json!({
            "title": title,
            "content": content
        }))
        .send()
        .await?
        .json::<PostCreated>()
        .await?;

    Ok(post_created)
}

#[derive(Debug, PartialEq, Clone)]
pub enum CreatePostResult {
    NotStarted,
    InProgress,
    Finished(PostCreated),
    Error(String),
}

#[component]
pub fn PostCreate() -> Element {
    let mut title = use_signal(|| "".to_string());
    let mut content = use_signal(|| "".to_string());

    let mut created_post_result = use_signal(|| CreatePostResult::NotStarted);
    // let mut create_post_future: Resource<Result<PostCreated>> =
    //     use_resource(move || async move { create_post(&title(), &content()).await });

    let create_post_action = move |event: MouseEvent| {
        event.prevent_default();

        info!(
            "create_post_action ->> - Title: {}, Content: {}",
            title, content
        );
        let _ = spawn(async move {
            created_post_result.set(CreatePostResult::InProgress);

            let post = create_post(&title(), &content()).await;
            match post {
                Ok(post) => created_post_result.set(CreatePostResult::Finished(post)),
                Err(e) => created_post_result.set(CreatePostResult::Error(e.to_string())),
            }
        });
    };

    rsx!(

        MyFormDiv {
            h1 { class: "text-2xl font-bold mb-6 text-center", "Create a New Post" }
            form { class: "space-y-4",
                FormInputText {
                    label: "Post title",
                    value: title(),
                    onchange: move |e: FormEvent| title.set(e.value().clone()),
                }

                FormInputText {
                    label: "Post content",
                    value: content(),
                    onchange: move |e: FormEvent| content.set(e.value().clone()),
                }

                FormButton { label: "Submit", onclick: create_post_action }
            }

            // Render created post based on condition
            RenderCreatePostResult { create_post_result: created_post_result() }
        }
    )
}

#[component]
fn RenderCreatePostResult(create_post_result: CreatePostResult) -> Element {
    rsx!(
        div { class: "mt-4",
            h2 { "Created Post Status" }
            match create_post_result {
                CreatePostResult::NotStarted => rsx! {
                    p { "No post has been created yet." }
                },
                CreatePostResult::InProgress => rsx! {
                    p { "Creating post..." }
                },
                CreatePostResult::Finished(post) => rsx! {
                    div {
                        h3 { "Post Created Successfully" }
                        p { "Title: {post.title}" }
                        p { "Content: {post.content}" }
                        p { "ID: {post.id}" }
                    }
                },
                CreatePostResult::Error(e) => rsx! {
                    div { class: "error",
                        h3 { "Error Creating Post" }
                        p { "An error occurred: {e}" }
                    }
                },
            }
        }
    )
}
