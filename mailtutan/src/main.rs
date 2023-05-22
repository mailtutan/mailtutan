use mailtutan_lib::{
    storage::{Connection, Memory},
    *,
};
use std::sync::Arc;
use std::sync::Mutex;

use tokio::runtime::Builder;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    let conn = Arc::new(Connection {
        storage: Mutex::new(Box::new(Memory::new())),
        ws_sender: broadcast::channel(100).0,
    });

    let mut tasks = vec![];

    tasks.push(runtime.spawn(api::serve(conn.clone())));
    tasks.push(runtime.spawn(smtp::serve(conn.clone())));

    for task in tasks {
        task.await.unwrap();
    }
}
