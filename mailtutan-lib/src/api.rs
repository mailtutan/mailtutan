use axum::extract::Extension;
use axum::{routing::delete, routing::get, Router};
use futures::lock::Mutex;

use crate::storage::{Connection, Storage};
use std::sync::Arc;

mod assets;
mod messages;
mod version;
mod websocket;

pub async fn serve(conn: Arc<Connection>) {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(assets::index_html))
        .route("/ws", get(websocket::websocket_handler))
        .route("/mailtutan-web_bg.wasm", get(assets::wasm))
        .route("/styles.css", get(assets::css))
        .route("/mailtutan-web.js", get(assets::js))
        .route("/api/messages", get(messages::index))
        .route("/api/messages/:id/source", get(messages::show_source))
        .route("/api/messages/:id/plain", get(messages::show_plain))
        .route("/api/messages/:id/html", get(messages::show_html))
        .route("/api/messages/:id/json", get(messages::show_json))
        .route("/api/messages/:id/eml", get(messages::show_eml))
        .route(
            "/api/messages/:id/parts/:cid",
            get(messages::download_attachment),
        )
        .route("/api/messages", delete(messages::delete_all))
        .route("/api/version", get(version::show))
        .layer(Extension(conn));

    println!("listening on http://0.0.0.0:1080");

    axum::Server::bind(&"0.0.0.0:1080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
