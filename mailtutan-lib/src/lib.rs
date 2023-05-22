#[macro_use]
extern crate lazy_static;

use tokio::sync::broadcast::{self, Sender};

pub mod api;
pub mod models;
pub mod smtp;
pub mod storage;
pub mod store;

lazy_static! {
    static ref WEBSOCKET_TX: Sender<String> = broadcast::channel(100).0;
}
