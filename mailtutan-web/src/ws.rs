use crate::Message;
use crate::MessageEvent;
use futures::StreamExt;
use gloo_net::websocket::{self, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub fn listen(messages: UseStateHandle<Vec<Message>>) {
    let messages = messages.clone();

    let ws = WebSocket::open("ws://127.0.0.1:1080/ws").unwrap();

    let (_, mut read) = ws.split();
    spawn_local(async move {
        while let Some(msg) = read.next().await {
            if let websocket::Message::Text(msg) = msg.unwrap() {
                let m: MessageEvent = serde_json::from_str(&msg).unwrap();

                let mut new_msg: Vec<Message> = (*messages).clone();

                new_msg.push(m.message);

                messages.set(new_msg);
            }
        }
    });
}
