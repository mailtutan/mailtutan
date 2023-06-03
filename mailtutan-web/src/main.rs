use yew::prelude::*;
use yewdux::prelude::*;

mod api;
mod component;

use serde::Deserialize;

#[derive(Clone, Properties, PartialEq, Deserialize, Default, Debug, Eq)]
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

#[derive(Clone, PartialEq, Deserialize, Debug, Eq)]
pub struct Attachment {
    pub cid: String,
    #[serde(rename = "type")]
    pub file_type: String,
    pub filename: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Store)]
struct State {
    messages: Vec<Message>,
    selected_message: Option<Message>,
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<component::root::Root>::new().render();
}
