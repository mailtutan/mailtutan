use crate::{models::Message, STORAGE};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Json;

pub async fn index() -> Json<Vec<Message>> {
    Json(STORAGE.lock().unwrap().list().to_vec())
}

pub async fn show_source(Path(id): Path<usize>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "text/plain;charset=utf-8")],
        STORAGE.lock().unwrap().get(id).source.clone(),
    )
}
pub async fn show_eml(Path(id): Path<usize>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "message/rfc822")],
        STORAGE.lock().unwrap().get(id).source.clone(),
    )
}

pub async fn show_json(Path(id): Path<usize>) -> Json<Message> {
    Json(STORAGE.lock().unwrap().get(id).clone())
}

pub async fn delete_all() -> Html<&'static str> {
    STORAGE.lock().unwrap().delete_all();
    Html("Ok")
}
