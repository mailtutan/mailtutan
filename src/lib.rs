#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use tokio::sync::broadcast::{self, Sender};

pub mod api;
pub mod models;
pub mod smtp;
pub mod store;

lazy_static! {
    static ref STORAGE: Mutex<store::MemoryStore> = Mutex::new(store::MemoryStore::new());
    static ref WEBSOCKET_TX: Sender<String> = broadcast::channel(100).0;
}
