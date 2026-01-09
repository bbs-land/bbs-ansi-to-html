//! Web server module for hosting the ANSI file converter application
//!
//! This module contains:
//! - Route handlers for the index page and file upload
//! - HTML templates for the UI
//! - Server startup logic

use axum::{
    Router,
    extract::Multipart,
    response::Html,
    routing::{get, post},
};
use ansi_to_html_rs::{convert_with_options, ConvertOptions};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

use crate::config::Config;

/// Start the web server with the given configuration.
///
/// # Arguments
///
/// * `config` - The application configuration containing port and wwwroot path
///
/// # Panics
///
/// Panics if the server fails to bind to the specified port or encounters
/// an error during operation.
pub async fn run(config: Config) {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/upload", post(upload_handler))
        .nest_service("/static", ServeDir::new(&config.wwwroot_path));

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("Server running at http://{}", addr);
    println!("Serving static files from: {}", config.wwwroot_path.display());

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Serve the index page with upload form
async fn index_handler() -> Html<String> {
    Html(INDEX_HTML.to_string())
}

/// Handle file uploads and convert to HTML
async fn upload_handler(mut multipart: Multipart) -> Html<String> {
    let mut file_content: Option<Vec<u8>> = None;
    let mut file_name = String::from("upload");
    let mut synchronet_enabled = false;
    let mut renegade_enabled = false;
    let mut utf8_input_enabled = false;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        match field.name() {
            Some("file") => {
                file_name = field.file_name().unwrap_or("upload").to_string();
                file_content = Some(field.bytes().await.unwrap_or_default().to_vec());
            }
            Some("synchronet") => {
                synchronet_enabled = true;
            }
            Some("renegade") => {
                renegade_enabled = true;
            }
            Some("utf8_input") => {
                utf8_input_enabled = true;
            }
            _ => {}
        }
    }

    let options = ConvertOptions {
        synchronet_ctrl_a: synchronet_enabled,
        renegade_pipe: renegade_enabled,
        utf8_input: utf8_input_enabled,
    };

    let content = match file_content {
        Some(bytes) => convert_with_options(&bytes, &options),
        None => "<p>No file uploaded</p>".to_string(),
    };

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - ANSI Viewer</title>
    <link rel="stylesheet" href="/static/style.css">
    <link rel="stylesheet" href="/static/ansi-display.css">
    <script src="/static/ansi-display.js"></script>
</head>
<body>
    <header>
        <h1>ANSI Viewer</h1>
        <nav><a href="/">← Upload Another File</a></nav>
    </header>
    <main class="viewer">
        <h2>{}</h2>
        <div class="ansi-container">
            {}
        </div>
    </main>
</body>
</html>"#,
        file_name, file_name, content
    );

    Html(html)
}

const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ANSI File Converter</title>
    <link rel="stylesheet" href="/static/style.css">
</head>
<body>
    <header>
        <h1>ANSI File Converter</h1>
    </header>
    <main>
        <h2>Upload a .msg or .ans file</h2>
        <form class="upload-form" action="/upload" method="post" enctype="multipart/form-data">
            <div class="file-input-wrapper">
                <label for="file">Select File:</label>
                <input type="file" id="file" name="file" accept=".msg,.ans,.txt">
            </div>
            <fieldset class="options-fieldset">
                <legend>Input Options</legend>
                <div class="checkbox-wrapper">
                    <input type="checkbox" id="utf8_input" name="utf8_input" value="1">
                    <label for="utf8_input">UTF-8 input (skip CP437 conversion, only convert control chars)</label>
                </div>
            </fieldset>
            <fieldset class="options-fieldset">
                <legend>BBS Color Code Options</legend>
                <div class="checkbox-wrapper">
                    <input type="checkbox" id="synchronet" name="synchronet" value="1">
                    <label for="synchronet">Synchronet Ctrl-A codes</label>
                </div>
                <div class="checkbox-wrapper">
                    <input type="checkbox" id="renegade" name="renegade" value="1">
                    <label for="renegade">Renegade pipe codes (|00-|23)</label>
                </div>
            </fieldset>
            <button type="submit">Convert &amp; View</button>
        </form>
        <p class="help-text">
            Supported formats: .msg, .ans (ANSI art files with CP437 encoding)
        </p>
    </main>
</body>
</html>"#;
