use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use futures::{sink::SinkExt, stream::StreamExt};

use std::{
    collections::HashSet,
    dbg,
    sync::{Arc, Mutex},
};

use tokio::sync::broadcast;

use crate::WEBSOCKET_TX;

pub fn state() -> Arc<AppState> {
    let (tx, _rx) = broadcast::channel(100);

    Arc::new(AppState { tx })
}

pub struct AppState {
    // Channel used to send messages to all connected clients.
    pub tx: broadcast::Sender<String>,
}

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    let mut rx = WEBSOCKET_TX.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // In any websocket error, break loop.
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });
}
