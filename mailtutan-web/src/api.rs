use crate::Message;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

pub fn fetch_messages() {
    // spawn_local(async move {
    //     let fetched_messages: Vec<Message> = Request::get("/api/messages")
    //         .send()
    //         .await
    //         .unwrap()
    //         .json()
    //         .await
    //         .unwrap();
    //     messages.set(fetched_messages);
    // });
}

pub fn delete_messages() {
    spawn_local(async move {
        Request::delete("/api/messages").send().await.unwrap();
    });
}
