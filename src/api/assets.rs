use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use std::fs;

// TODO: security issue
// TODO: put the file in compiled binary
pub async fn show(Path(name): Path<String>) -> impl IntoResponse {
    let file_path = format!("{}/public/{}", env!("CARGO_MANIFEST_DIR"), name);

    let content_type = match name.split(".").last().unwrap() {
        "css" => "text/css",
        "js" => "application/javascript;charset=utf-8",
        _ => "text/plain",
    };

    (
        StatusCode::OK,
        [("Content-Type", content_type)],
        fs::read_to_string(file_path).unwrap(),
    )
}

pub async fn index_html() -> Html<String> {
    let file_path = format!("{}/public/index.html", env!("CARGO_MANIFEST_DIR"));

    Html(fs::read_to_string(file_path).unwrap())
}
