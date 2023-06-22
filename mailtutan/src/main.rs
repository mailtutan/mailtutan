use std::sync::Arc;

use clap::Parser;
use mailtutan_lib::*;

mod config;
use crate::config::StorageType;
use config::Config;
use mailtutan_lib::storage::Storage;
use std::sync::RwLock;

use tokio::sync::broadcast;
use tokio::{self, runtime::Builder, signal};

#[tokio::main]
async fn main() {
    let config = Config::parse();

    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    let state = {
        let storage: Box<RwLock<dyn Storage + 'static>> = match config.storage {
            StorageType::Memory => {
                Box::new(RwLock::new(storage::Memory::new(config.messages_limit)))
            }
            StorageType::Maildir => Box::new(RwLock::new(storage::Memdir::new(
                config.messages_limit,
                config.maildir_path,
            ))),
        };
        Arc::new(AppState {
            storage,
            channel: broadcast::channel(100).0,
            messages_limit: config.messages_limit,
            smtp_auth_username: config.smtp_auth_username.clone(),
            smtp_auth_password: config.smtp_auth_password.clone(),
            http_auth_username: config.http_username,
            http_auth_password: config.http_password,
            web_hook: config.web_hook,
        })
    };

    let api_server = api::Builder::new()
        .http_auth(config.http_auth)
        .bind((config.ip, config.http_port).into())
        .with_state(state.clone())
        .build();

    let smtp_server = smtp::Builder::new()
        .with_state(state.clone())
        .with_ssl(config.smtp_cert_path, config.smtp_key_path)
        .with_auth(config.smtp_auth_username.is_some() && config.smtp_auth_password.is_some())
        .bind((config.ip, config.smtp_port).into())
        .build();

    let web_hook_worker = web_hook::Worker::new(state.clone());

    tokio::select! {
        _ = runtime.spawn(api_server.serve()) => {
        }
        _ = runtime.spawn(smtp_server.serve()) => {
        }
        _ = runtime.spawn(web_hook_worker.serve()) => {
        }
        _ = signal::ctrl_c() => {
        }
    }
    runtime.shutdown_background();
}
