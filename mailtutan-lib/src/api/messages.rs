use crate::models::Message;
use crate::storage::Connection;
use axum::extract::Extension;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

pub async fn index(Extension(conn): Extension<Arc<Connection>>) -> Json<Vec<Message>> {
    Json(conn.storage.lock().unwrap().list().to_vec())
}

pub async fn show_source(
    Path(id): Path<usize>,
    Extension(conn): Extension<Arc<Connection>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "text/plain;charset=utf-8")],
        conn.storage.lock().unwrap().get(id).source.clone(),
    )
}

pub async fn show_plain(
    Path(id): Path<usize>,
    Extension(conn): Extension<Arc<Connection>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "text/plain;charset=utf-8")],
        conn.storage
            .lock()
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
    Extension(conn): Extension<Arc<Connection>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "text/html;charset=utf-8")],
        conn.storage
            .lock()
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
    Extension(conn): Extension<Arc<Connection>>,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "message/rfc822")],
        conn.storage.lock().unwrap().get(id).source.clone(),
    )
}

pub async fn download_attachment(
    Path((id, cid)): Path<(usize, String)>,
    Extension(conn): Extension<Arc<Connection>>,
) -> impl IntoResponse {
    for attachment in &conn.storage.lock().unwrap().get(id).attachments {
        if attachment.cid == cid {
            return (
                StatusCode::OK,
                [("Content-Disposition", "attachment; filename=\"attachment\"")],
                attachment.body.clone(),
            );
        }
    }

    (StatusCode::OK, [("Content-Type", "message/rfc822")], vec![])
}

pub async fn show_json(
    Path(id): Path<usize>,
    Extension(conn): Extension<Arc<Connection>>,
) -> Json<Message> {
    Json(conn.storage.lock().unwrap().get(id).clone())
}

pub async fn delete_all(Extension(conn): Extension<Arc<Connection>>) -> Html<&'static str> {
    conn.storage.lock().unwrap().delete_all();
    Html("Ok")
}
