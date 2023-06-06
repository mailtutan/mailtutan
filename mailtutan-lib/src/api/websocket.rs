use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};

use futures::{sink::SinkExt, stream::StreamExt};

use crate::APP;

pub async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(websocket)
}

async fn websocket(stream: WebSocket) {
    let (mut sender, _) = stream.split();

    let mut rx = APP.get().unwrap().lock().unwrap().ws_sender.subscribe();

    tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
}
