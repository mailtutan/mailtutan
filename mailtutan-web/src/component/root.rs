use yew::prelude::*;

use crate::api;
use crate::component::header::Header;
use crate::component::message_list::MessageList;
use crate::component::message_view::MessageView;
use crate::Message;
use crate::MessageEvent;
use crate::State;
use futures::StreamExt;
use gloo_dialogs::confirm;
use gloo_net::http::Request;
use gloo_net::websocket::{self, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use yewdux::prelude::*;

use std::sync::Once;
use web_sys::HtmlTableCellElement;

static WS_ONCE: Once = Once::new();

#[function_component]
pub fn Root() -> Html {
    let (state, dispatch) = use_store::<State>();

    // clear messages
    let clear_onclick = {
        let dispatch = dispatch.clone();

        Callback::from(move |_: MouseEvent| {
            if confirm("Do you want to clear all messages?") {
                api::delete_messages();

                dispatch.reduce_mut(|state| {
                    state.messages.clear();
                    state.selected_message = None;
                });
            }
        })
    };

    // click on message in message list
    let onclick = {
        #[allow(clippy::redundant_clone)]
        let state = state.clone();
        let dispatch = dispatch.clone();

        Callback::from(move |e: MouseEvent| {
            let element: HtmlTableCellElement = e.target_unchecked_into();
            let id = element
                .parent_element()
                .unwrap()
                .get_attribute("data-message-id")
                .unwrap()
                .parse::<usize>()
                .unwrap();

            if let Some(message) = state.messages.get(&id) {
                dispatch.reduce_mut(|state| state.selected_message = Some(message.clone()));
            }
        })
    };

    // load existing messages
    {
        let dispatch = dispatch.clone();

        use_effect_with_deps(
            move |_| {
                // api::fetch_messages();
                spawn_local(async move {
                    let fetched_messages: Vec<Message> = Request::get("/api/messages")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();

                    dispatch.reduce_mut(|state| {
                        for message in fetched_messages {
                            state.messages.insert(message.id.unwrap(), message.clone());
                        }
                    });
                });
            },
            (),
        );
    }

    // start web socket
    {
        WS_ONCE.call_once(move || {
            let ws_url = {
                let href = web_sys::window().unwrap().location().href().unwrap();
                let mut url = url::Url::parse(&href).unwrap();
                if url.scheme() == "https" {
                    url.set_scheme("wss").unwrap();
                } else {
                    url.set_scheme("ws").unwrap();
                }
                url.set_path("/ws");
                url
            };

            let ws = WebSocket::open(ws_url.as_ref()).unwrap();

            let (_, mut read) = ws.split();

            spawn_local(async move {
                while let Some(msg) = read.next().await {
                    if let Ok(websocket::Message::Text(msg)) = msg {
                        if let Ok(m) = serde_json::from_str::<MessageEvent>(&msg) {
                            dispatch.reduce_mut(|state| {
                                state.messages.insert(m.message.id.unwrap(), m.message)
                            });
                        }
                    }
                }
            });
        });
    }

    html! {
        <>
            <Header clear_onclick={clear_onclick} />
            <div class="main">
                <MessageList {onclick} />
                <MessageView />
            </div>
        </>
    }
}
