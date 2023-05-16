use crate::{models::Message, STORAGE};
use axum::response::Html;
use axum::Json;

pub async fn index() -> Json<Vec<Message>> {
    Json(STORAGE.lock().unwrap().list().to_vec())
}

pub async fn delete_all() -> Html<&'static str> {
    STORAGE.lock().unwrap().delete_all();
    Html("Ok")
}
