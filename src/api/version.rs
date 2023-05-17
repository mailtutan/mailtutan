use axum::response::Html;

use crate::api::websocket::AppState;
use axum::extract::State;
use std::sync::Arc;

// pub async fn show() -> Html<&'static str> {
//     Html(env!("CARGO_PKG_VERSION"))
// }

pub async fn show(State(state): State<Arc<AppState>>) -> Html<&'static str> {
    state.tx.clone().send("something".to_owned());

    Html(env!("CARGO_PKG_VERSION"))
}
