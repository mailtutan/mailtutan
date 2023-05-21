use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

mod emails_list;
mod header;
mod message_view;
mod resizer;

use emails_list::EmailsList;
use header::Header;
use message_view::MessageView;
use resizer::Resizer;

use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Default, Debug)]
pub struct Message {
    pub id: Option<usize>,
    pub sender: String,
    pub recipients: Vec<String>,
    pub subject: String,
    pub created_at: String,
    pub attachments: Vec<Attachment>,
    pub formats: Vec<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MessageEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub message: Message,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Attachment {
    pub cid: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub filename: String,
}

#[function_component]
fn App() -> Html {
    let messages = use_state(|| vec![]);
    {
        let messages = messages.clone();
        use_effect_with_deps(
            move |_| {
                let messages = messages.clone();

                spawn_local(async move {
                    let fetched_messages: Vec<Message> = Request::get("/api/messages")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    messages.set(fetched_messages);
                });
                || ()
            },
            (),
        );
    }

    {
        let messages = messages.clone();
        use futures::StreamExt;
        use gloo_net::websocket;
        use gloo_net::websocket::futures::WebSocket;
        use serde_json;
        use wasm_bindgen_futures::spawn_local;

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

    html! {
        <>
            <Header/>
            <EmailsList messages={(*messages).clone()}/>
            <Resizer/>
            <MessageView/>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
