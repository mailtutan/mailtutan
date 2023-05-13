use crate::{models::message::Message, STORAGE};
use axum::Json;

pub async fn index() -> Json<Vec<Message>> {
    Json(STORAGE.lock().unwrap().list().to_vec())
}
