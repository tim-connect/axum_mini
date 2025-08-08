//! # axum_mini
//!
//! Lightweight HTML minifier middleware for [axum](https://crates.io/crates/axum) applications.
//!
//! This crate provides a simple middleware function that intercepts HTTP responses and minifies
//! the HTML content before sending it to the client. By reducing HTML size, it helps improve
//! bandwidth usage and page load times.
//!
//! ## Features
//!
//! - Buffers full HTTP response body to process HTML content.
//! - Uses [`minify-html`](https://crates.io/crates/minify-html) to perform aggressive HTML, CSS, and JS minification.
//! - Works seamlessly as an axum middleware layer.
//!
//! ## Usage
//!
//! Apply the middleware to your axum router:
//!
//! ```rust
//! use axum::{middleware, Router};
//! use axum_mini::html_minifier;
//!
//! #[tokio::main]
//! async fn main() {
//!     let app = Router::new()
//!         .route("/", axum::routing::get(|| async { "<h1>Hello World!</h1>" }))
//!         .layer(middleware::from_fn(html_minifier));
//!
//!     axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
//!         .serve(app.into_make_service())
//!         .await
//!         .unwrap();
//! }
//! ```
//!
//! ## How it works
//!
//! 1. The middleware buffers the entire HTTP response body.
//! 2. It checks if the `Content-Type` header contains `text/html`.
//! 3. If so, it applies HTML minification using `minify-html` with a preset configuration.
//! 4. The minified HTML is then sent as the response body.
//! 5. Non-HTML responses are forwarded without modification.
//!
//! ## Configuration
//!
//! The minifier uses a fixed configuration optimized for general use, including removal of comments,
//! minification of embedded CSS and JS, and whitespace optimization.
//!
//! ## License
//!
//! This crate is licensed under the MIT License.
//!


use axum::{
    body::{Body, Bytes},
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use minify_html::{minify, Cfg};
use http_body_util::BodyExt;

/// Middleware that minifies HTML responses.
pub async fn html_minifier(req: Request<Body>, next: Next) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Buffer entire response body
    let (parts, body) = next.run(req).await.into_parts();
    let response_bytes = response_buffer(body)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR,  e))?;

    // Check content-type header
    let is_html = parts.headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(|ct| ct.contains("text/html"))
        .unwrap_or(false);

    // Minify if HTML
    let final_body = if is_html {
        let mut cfg = Cfg::new();
        cfg.allow_removing_spaces_between_attributes = true;
        cfg.minify_css = true;
        cfg.minify_js = true;
        cfg.remove_bangs = true;
        cfg.remove_processing_instructions = true;
        cfg.keep_comments = false;

        Bytes::from(minify(&response_bytes, &cfg))
    } else {
        response_bytes
    };

    let response = Response::from_parts(parts, Body::from(final_body));
    Ok(response)
}

/// Helper to read the entire body to bytes
async fn response_buffer<B>(body: B) -> Result<axum::body::Bytes, String>
where
    B: axum::body::HttpBody<Data = axum::body::Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {return Err(format!("failed to read response body: {err}"));}
    };
    Ok(bytes)
}
