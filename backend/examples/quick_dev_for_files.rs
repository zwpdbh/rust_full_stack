#![allow(unused)] // For beginning only.

use anyhow::Result;
use reqwest::multipart;
use serde_json::json;

/// cargo watch -q -c -w examples/ -x "run --example quick_dev_for_files"
/// or
/// cargo run --example quick_dev_for_files
#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let form = multipart::Form::new()
        .file(
            "file_1",
            "/home/zw/code/rust_programming/loco_hello/README.md",
        )
        .await?
        .file(
            "file_2",
            "/home/zw/code/rust_programming/loco_hello/Cargo.toml",
        )
        .await?;

    // Send the POST request with multipart form data
    let response = client
        .post("http://localhost:5150/api/files/upload/1")
        .multipart(form)
        .send()
        .await?;

    // Print the response
    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());
    println!("Body: {}", response.text().await?);

    Ok(())
}
