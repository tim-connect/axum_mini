//! axum_mini – Lightweight HTML minifier middleware for Axum.
//!
//! See the [README](https://crates.io/crates/axum_mini) for full usage and examples.

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
