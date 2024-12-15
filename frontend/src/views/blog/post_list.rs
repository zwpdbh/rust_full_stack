#![allow(non_snake_case)]
use super::post_create::PostCreated;
use crate::error::Result;
use dioxus::prelude::*;
use dioxus_sortable::{NullHandling, PartialOrdBy, SortBy, Sortable};
use reqwest::Client;
use tracing::info;

async fn get_posts() -> Result<Vec<PostCreated>> {
    let client = Client::new();
    let posts = client
        .get("http://localhost:5150/api/posts")
        .send()
        .await?
        .json::<Vec<PostCreated>>()
        .await?;

    Ok(posts)
}

#[derive(Debug, PartialEq, Clone)]
pub enum GetPostsResult {
    NotStarted,
    InProgress,
    Finished(Vec<PostCreated>),
    Error(String),
}

/// This is the field we want to sort by. Each variant corresponds to a column in our table or field in our Person struct. Keep it simple, use `{struct}Field`.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
enum PostSortField {
    Id,
    #[default]
    Title,
}

/// This trait decides how our rows are sorted
impl PartialOrdBy<PostCreated> for PostSortField {
    fn partial_cmp_by(&self, a: &PostCreated, b: &PostCreated) -> Option<std::cmp::Ordering> {
        match self {
            PostSortField::Id => a.id.partial_cmp(&b.id),
            PostSortField::Title => a.title.partial_cmp(&b.title),
        }
    }
}

/// This trait decides how fields (columns) may be sorted
impl Sortable for PostSortField {
    fn sort_by(&self) -> Option<SortBy> {
        use PostSortField::*;
        match self {
            Id => SortBy::increasing_or_decreasing(),
            Title => SortBy::increasing_or_decreasing(),
        }
    }

    fn null_handling(&self) -> NullHandling {
        match self {
            _ => NullHandling::Last,
        }
    }
}

#[component]
pub fn PostList() -> Element {
    use_context_provider(|| Signal::new(PostSortField::Id));
    let mut get_posts_result = use_signal(|| GetPostsResult::NotStarted);

    let get_posts_action = move |_event: MouseEvent| {
        info!("Getting posts");
        let _ = spawn(async move {
            get_posts_result.set(GetPostsResult::InProgress);

            let posts = get_posts().await;
            match posts {
                Ok(posts) => get_posts_result.set(GetPostsResult::Finished(posts)),
                Err(e) => get_posts_result.set(GetPostsResult::Error(e.to_string())),
            }
        });
    };

    rsx!(
        h1 { "page for list posts" }

        div {
            button { class: "button", onclick: get_posts_action, "Get Posts" }
        }

        RenderGetPostsResult { get_posts_result: get_posts_result() }
    )
}

#[component]
fn RenderGetPostsResult(get_posts_result: GetPostsResult) -> Element {
    rsx!(
        div { class: "mt-4",
            match get_posts_result {
                GetPostsResult::NotStarted => rsx! { "No posts has been fetched" },
                GetPostsResult::InProgress => rsx! {
                    p { "Getting posts..." }
                },
                GetPostsResult::Finished(posts) => rsx! {
                    p { "successfully get posts: {posts.len()}" }

                    table {
                        tr {
                            th { "Id" }
                            th { "Title" }
                            th { "Content" }
                        }

                        for post in posts {
                            tr {
                                td { "{post.id}" }
                                td { "{post.title}" }
                                td { "{post.content}" }
                            }
                        }
                    }
                },
                GetPostsResult::Error(e) => rsx! {
                    h3 { "Error Creating Post" }
                    p { "An error occurred: {e}" }
                },
            }
        }
    )
}
