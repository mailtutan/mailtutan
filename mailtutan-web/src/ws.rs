use crate::Message;
use crate::MessageEvent;
use crate::State;
use futures::StreamExt;
use gloo_net::websocket::{self, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewdux::prelude::*;

pub fn listen() {
    // let ws = WebSocket::open("ws://127.0.0.1:1080/ws").unwrap();
    //
    // let (_, mut read) = ws.split();
    // spawn_local(async move {
    //     while let Some(msg) = read.next().await {
    //         if let Ok(websocket::Message::Text(msg)) = msg {
    //             if let Ok(m) = serde_json::from_str::<MessageEvent>(&msg) {
    //                 let (state, dispatch) = use_store::<State>();
    //
    //                 log::info!("a message received");
    //                 // log::info!("{:?}", m.message.id.ok_or("none"));
    //                 //
    //                 // let mut new_msg: Vec<Message> = (*messages).clone();
    //                 //
    //                 // new_msg.push(m.message);
    //                 //
    //                 // messages.set(new_msg);
    //             }
    //         }
    //     }
    // });
}
