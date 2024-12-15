use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn cors_layer() -> CorsLayer {
    CorsLayer::new()
        // Allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        // Allow requests from any origin
        .allow_origin(Any)
        // Allow sending any headers
        .allow_headers(Any)
        // Allow credentials (cookies, authorization headers, etc.)
        .allow_credentials(false)
    // Optionally, you can specify allowed headers explicitly
    // .allow_headers([HeaderName::from_static("content-type")])
}
