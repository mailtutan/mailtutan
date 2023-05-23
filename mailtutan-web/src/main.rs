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

#[derive(Clone, Properties, PartialEq, Deserialize, Default, Debug)]
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

use web_sys::HtmlTableCellElement;
// use web_sys::HtmlTableRowElement;

#[function_component]
fn App() -> Html {
    let messages = use_state(|| vec![]);
    let selected_message = use_state(|| None);

    // let onclick = Callback::from(move |e: MouseEvent| {});

    let temp_message = selected_message.clone();
    let onclick = Callback::from(move |e: MouseEvent| {
        // let messages = messages.clone();
        // let selected_message = selected_message.clone();

        log::info!("just clicked");

        // let a: HtmlTableRowElement = e.target_unchecked_into();
        let element: HtmlTableCellElement = e.target_unchecked_into();
        let id = element
            .parent_element()
            .unwrap()
            .get_attribute("data-message-id")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        log::info!("{}", id);
        // let msg = (*messages.get(id).unwrap()).id.unwrap();
        //
        let m: usize = 1;
        temp_message.set(Some(m));

        // message.set(Some(((*messages).get(id)).clone()));

        // message.set(Some(((*messages).get(id)).clone()));
    });

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
            <EmailsList messages={(*messages).clone()} onclick={onclick} />
            <Resizer/>
            <MessageView message={(*selected_message).clone()} />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
