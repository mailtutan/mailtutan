use crate::storage::Connection;
use axum::extract::Extension;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use std::sync::Arc;

use futures::{sink::SinkExt, stream::StreamExt};

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    Extension(conn): Extension<Arc<Connection>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, conn))
}

async fn websocket(stream: WebSocket, conn: Arc<Connection>) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, _) = stream.split();

    let mut rx = conn.ws_sender.subscribe();

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
}
