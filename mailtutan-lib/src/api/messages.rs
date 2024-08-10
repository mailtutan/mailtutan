use std::sync::Arc;

use crate::models::Message;
use crate::AppState;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::{Html, Response};
use axum::Json;

pub async fn index(State(state): State<Arc<AppState>>) -> Json<Vec<Message>> {
    Json(state.storage.read().unwrap().list().to_vec())
}

pub async fn show_source(Path(id): Path<usize>, State(state): State<Arc<AppState>>) -> Response {
    if let Some(msg) = state.storage.read().expect("unpoisoned lock").get(id) {
        (
            StatusCode::OK,
            [("Content-Type", "text/plain;charset=utf-8")],
            msg.source,
        )
            .into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn delete(
    Path(id): Path<usize>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    state.storage.write().expect("unpoisoned lock").remove(id);

    StatusCode::OK
}

pub async fn show_plain(Path(id): Path<usize>, State(state): State<Arc<AppState>>) -> Response {
    if let Some(msg) = state
        .storage
        .read()
        .expect("unpoisoned lock")
        .get(id)
        .and_then(|m| m.plain)
    {
        (
            StatusCode::OK,
            [("Content-Type", "text/plain;charset=utf-8")],
            msg,
        )
            .into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn show_html(Path(id): Path<usize>, State(state): State<Arc<AppState>>) -> Response {
    if let Some(msg) = state
        .storage
        .read()
        .expect("unpoisoned lock")
        .get(id)
        .and_then(|m| m.html)
    {
        (
            StatusCode::OK,
            [("Content-Type", "text/html;charset=utf-8")],
            msg,
        )
            .into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn show_eml(Path(id): Path<usize>, State(state): State<Arc<AppState>>) -> Response {
    if let Some(msg) = state.storage.read().expect("unpoisoned lock").get(id) {
        (
            StatusCode::OK,
            [("Content-Type", "message/rfc822")],
            msg.source,
        )
            .into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn download_attachment(
    Path((id, cid)): Path<(usize, String)>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    if let Some(attachment) = state
        .storage
        .read()
        .expect("unpoisoned lock")
        .get(id)
        .and_then(|msg| msg.attachments.iter().find(|a| a.cid == cid).cloned())
    {
        (
            StatusCode::OK,
            [(
                "Content-Disposition",
                format!("attachment; filename=\"{}\"", attachment.filename),
            )],
            attachment.body,
        )
    } else {
        (
            StatusCode::OK,
            [("Content-Type", "message/rfc822".to_string())],
            vec![],
        )
    }
}

pub async fn show_json(Path(id): Path<usize>, State(state): State<Arc<AppState>>) -> Response {
    if let Some(msg) = state.storage.read().expect("unpoisoned lock").get(id) {
        Json(msg).into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}

pub async fn delete_all(State(state): State<Arc<AppState>>) -> Html<&'static str> {
    state.storage.write().expect("unpoisoned lock").delete_all();
    Html("Ok")
}
