use crate::models::message::Message;
use axum::Json;

pub async fn index() -> Json<Vec<Message>> {
    Json(vec![Message {
        from: "mohsen".to_owned(),
        to: "something".to_owned(),
        data: "something".to_owned(),
    }])
}
