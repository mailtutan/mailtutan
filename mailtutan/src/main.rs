use mailtutan_lib::{
    storage::{Connection, Memory},
    *,
};

mod config;
use config::Config;

use std::sync::Arc;
use std::sync::Mutex;

use tokio::runtime::Builder;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let cfg = Config::from_env_and_args();

    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    let conn = Arc::new(Connection {
        storage: Mutex::new(Box::new(Memory::new())),
        ws_sender: broadcast::channel(100).0,
    });

    let mut tasks = vec![];

    tasks.push(runtime.spawn(api::serve(conn.clone(), cfg.get_api_uri())));
    tasks.push(runtime.spawn(smtp::serve(conn.clone(), cfg.get_smtp_uri())));

    for task in tasks {
        task.await.unwrap();
    }
}
