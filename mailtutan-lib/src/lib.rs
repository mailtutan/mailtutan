pub mod api;
pub mod auth;
pub mod models;
pub mod smtp;
pub mod storage;
use once_cell::sync::OnceCell;
use storage::Storage;
use tokio::sync::broadcast::Sender;

static APP: OnceCell<Mutex<Mailtutan>> = OnceCell::new();

use std::{net::Ipv4Addr, sync::Mutex};

pub struct Mailtutan {
    pub ip: Ipv4Addr,
    pub http_port: u16,
    pub smtp_port: u16,
    pub storage: Box<dyn Storage + 'static>,
    pub ws_sender: Sender<String>,
    pub http_auth: bool,
    pub http_username: String,
    pub http_password: String,
    pub messages_limit: usize,
    pub smtp_cert_path: Option<String>,
    pub smtp_key_path: Option<String>,
}

impl Mailtutan {
    pub fn get_api_uri(&self) -> String {
        format!("{}:{}", self.ip, self.http_port)
    }

    pub fn get_smtp_uri(&self) -> String {
        format!("{}:{}", self.ip, self.smtp_port)
    }

    pub fn init(self) {
        APP.get_or_init(|| Mutex::new(self));
    }
}
