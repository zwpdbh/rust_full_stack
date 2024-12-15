#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

/// cargo watch -q -c -w examples/ -x "run --example quick_dev_for_posts"
/// or
/// cargo run --example quick_dev_for_posts
#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:5150")?;

    // hc.do_get("/index.html").await?.print().await?;

    let req_get_posts = hc.do_get("/api/posts");
    let _ = req_get_posts.await?.print().await?;

    let req_create_post = hc.do_post(
        "/api/posts",
        json!({
            "title": "post 01",
            "content": "post 01 content"
        }),
    );
    let _ = req_create_post.await?.print().await?;

    let req_create_post = hc.do_post(
        "/api/posts",
        json!({
            "title": "post 02",
            "content": "post 02 content"
        }),
    );
    let _ = req_create_post.await?.print().await?;

    let req_get_posts = hc.do_get("/api/posts");
    let _ = req_get_posts.await?.print().await?;

    Ok(())
}
