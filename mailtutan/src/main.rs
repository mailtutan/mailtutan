use mailtutan_lib::*;

mod config;
use config::Config;

use tokio::{self, runtime::Builder, signal};

#[tokio::main]
async fn main() {
    Config::from_env_and_args().build().init();

    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    tokio::select! {
        _ = runtime.spawn(api::serve()) => {
        }
        _ = runtime.spawn(smtp::serve()) => {
        }
        _ = signal::ctrl_c() => {
        }
    }
    runtime.shutdown_background();
}
