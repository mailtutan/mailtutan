use mailtutan_lib::*;

mod config;
use config::Config;

use tokio::runtime::Builder;

#[tokio::main]
async fn main() {
    Config::from_env_and_args().build().init();

    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    let mut tasks = vec![];

    tasks.push(runtime.spawn(api::serve()));
    tasks.push(runtime.spawn(smtp::serve()));

    for task in tasks {
        task.await.unwrap();
    }
}
