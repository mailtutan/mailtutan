use crate::models::Message;
use crate::APP;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Json;

pub async fn index() -> Json<Vec<Message>> {
    Json(APP.get().unwrap().lock().unwrap().storage.list().to_vec())
}

pub async fn show_source(Path(id): Path<usize>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "text/plain;charset=utf-8")],
        APP.get()
            .unwrap()
            .lock()
            .unwrap()
            .storage
            .get(id)
            .source
            .clone(),
    )
}

pub async fn delete(Path(id): Path<usize>) -> impl IntoResponse {
    APP.get().unwrap().lock().unwrap().storage.remove(id);

    StatusCode::OK
}

pub async fn show_plain(Path(id): Path<usize>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "text/plain;charset=utf-8")],
        APP.get()
            .unwrap()
            .lock()
            .unwrap()
            .storage
            .get(id)
            .plain
            .as_ref()
            .unwrap()
            .clone(),
    )
}

pub async fn show_html(Path(id): Path<usize>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "text/html;charset=utf-8")],
        APP.get()
            .unwrap()
            .lock()
            .unwrap()
            .storage
            .get(id)
            .html
            .as_ref()
            .unwrap()
            .clone(),
    )
}

pub async fn show_eml(Path(id): Path<usize>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "message/rfc822")],
        APP.get()
            .unwrap()
            .lock()
            .unwrap()
            .storage
            .get(id)
            .source
            .clone(),
    )
}

pub async fn download_attachment(Path((id, cid)): Path<(usize, String)>) -> impl IntoResponse {
    for attachment in &APP
        .get()
        .unwrap()
        .lock()
        .unwrap()
        .storage
        .get(id)
        .attachments
    {
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

pub async fn show_json(Path(id): Path<usize>) -> Json<Message> {
    Json(APP.get().unwrap().lock().unwrap().storage.get(id).clone())
}

pub async fn delete_all() -> Html<&'static str> {
    APP.get().unwrap().lock().unwrap().storage.delete_all();
    Html("Ok")
}
