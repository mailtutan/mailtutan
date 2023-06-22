use std::sync::Arc;

use crate::models::Message;
use crate::AppState;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Json;

pub async fn index(State(state): State<Arc<AppState>>) -> Json<Vec<Message>> {
    Json(state.storage.read().unwrap().list().to_vec())
}

pub async fn show_source(
    Path(id): Path<usize>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "text/plain;charset=utf-8")],
        state.storage.read().unwrap().get(id).source,
    )
}

pub async fn delete(
    Path(id): Path<usize>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    state.storage.write().unwrap().remove(id);

    StatusCode::OK
}

pub async fn show_plain(
    Path(id): Path<usize>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "text/plain;charset=utf-8")],
        state
            .storage
            .read()
            .unwrap()
            .get(id)
            .plain
            .as_ref()
            .unwrap()
            .clone(),
    )
}

pub async fn show_html(
    Path(id): Path<usize>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "text/html;charset=utf-8")],
        state
            .storage
            .read()
            .unwrap()
            .get(id)
            .html
            .as_ref()
            .unwrap()
            .clone(),
    )
}

pub async fn show_eml(
    Path(id): Path<usize>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "message/rfc822")],
        state.storage.read().unwrap().get(id).source,
    )
}

pub async fn download_attachment(
    Path((id, cid)): Path<(usize, String)>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    for attachment in &state.storage.read().unwrap().get(id).attachments {
        if attachment.cid == cid {
            return (
                StatusCode::OK,
                [(
                    "Content-Disposition",
                    format!("attachment; filename=\"{}\"", attachment.filename),
                )],
                attachment.body.clone(),
            );
        }
    }

    (
        StatusCode::OK,
        [("Content-Type", "message/rfc822".to_string())],
        vec![],
    )
}

pub async fn show_json(Path(id): Path<usize>, State(state): State<Arc<AppState>>) -> Json<Message> {
    Json(state.storage.read().unwrap().get(id))
}

pub async fn delete_all(State(state): State<Arc<AppState>>) -> Html<&'static str> {
    state.storage.write().unwrap().delete_all();
    Html("Ok")
}
