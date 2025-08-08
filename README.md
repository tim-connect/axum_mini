# axum_mini

**Lightweight HTML minifier middleware for [Axum](https://crates.io/crates/axum) applications.**

This crate provides a simple middleware function that intercepts HTTP responses and minifies the HTML content before sending it to the client. By reducing HTML size, it helps improve bandwidth usage and page load times.

---

## ✨ Features

- Buffers the full HTTP response body to process HTML content.
- Uses [`minify-html`](https://crates.io/crates/minify-html) to perform aggressive HTML, CSS, and JS minification.
- Integrates easily as an Axum middleware layer.

---

## 🚀 Usage

Add the crate to your `Cargo.toml`:

```toml
axum_mini = "0.1"
```

Apply the middleware to your Axum router:
```
use axum::{middleware, Router};
use axum_mini::html_minifier;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", axum::routing::get(|| async { "<h1>Hello World!</h1>" }))
        .layer(middleware::from_fn(html_minifier));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

```

## 🛠️ How It Works

    The middleware buffers the entire HTTP response body.
    It checks if the Content-Type header contains text/html.
    If so, it applies HTML minification using minify-html with a preset configuration.
    The minified HTML is then sent as the response body.
    Non-HTML responses are forwarded without modification.

⚙️ Configuration

## The minifier uses a fixed configuration optimized for general use:

    Removes comments
    Minifies embedded CSS and JavaScript
    Optimizes whitespace and tag formatting

## 📄 License

**This crate is licensed under the MIT License. Thank goodness!**
