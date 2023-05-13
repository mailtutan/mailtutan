use axum::response::Html;

pub async fn version_handler() -> Html<&'static str> {
    Html(env!("CARGO_PKG_VERSION"))
}
