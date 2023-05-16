#[macro_use]
extern crate lazy_static;

mod api;
mod models;
mod smtp;
mod store;

use std::sync::Mutex;
use tokio::runtime::Builder;

lazy_static! {
    static ref STORAGE: Mutex<store::MemoryStore> = Mutex::new(store::MemoryStore::new());
}

#[tokio::main]
async fn main() {
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    let mut tasks = vec![];

    tasks.push(runtime.spawn(api::serve()));
    tasks.push(runtime.spawn(smtp::serve()));

    for task in tasks {
        task.await.unwrap();
    }
}
