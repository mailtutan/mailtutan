use mailtutan_lib::{
    storage::{Connection, Memory},
    *,
};
use std::sync::Arc;
use std::sync::Mutex;

use tokio::runtime::Builder;

#[tokio::main]
async fn main() {
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    let storage = Arc::new(Connection {
        storage: Mutex::new(Box::new(Memory::new())),
    });

    let mut tasks = vec![];

    tasks.push(runtime.spawn(api::serve(storage.clone())));
    tasks.push(runtime.spawn(smtp::serve(storage.clone())));

    for task in tasks {
        task.await.unwrap();
    }
}
