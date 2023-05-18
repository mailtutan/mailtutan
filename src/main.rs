#[macro_use]
extern crate lazy_static;

use mailtutan::*;

use tokio::runtime::Builder;

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
