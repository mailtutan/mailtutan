use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;

static HTML: &[u8] = include_bytes!("../../static/index.html");
static CSS: &[u8] = include_bytes!("../../static/styles.css");
static JS: &[u8] = include_bytes!("../../static/mailtutan-web.js");
static WASM: &[u8] = include_bytes!("../../static/mailtutan-web_bg.wasm");

pub async fn js() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "application/javascript;charset=utf-8")],
        JS,
    )
}

pub async fn wasm() -> impl IntoResponse {
    (StatusCode::OK, [("Content-Type", "application/wasm")], WASM)
}

pub async fn css() -> impl IntoResponse {
    (StatusCode::OK, [("Content-Type", "text/css")], CSS)
}

pub async fn index_html() -> Html<&'static [u8]> {
    Html(HTML)
}
