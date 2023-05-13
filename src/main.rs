use anyhow::Result;
use async_trait::async_trait;
#[macro_use]
extern crate lazy_static;
use models::message::Message;
use tokio::io::{self, AsyncRead, AsyncReadExt};
use tokio::runtime::Builder;

use rust_smtp_server::backend::{Backend, MailOptions, Session};
use rust_smtp_server::server::Server;

struct MyBackend;

#[derive(Default)]
struct MySession {
    pub message: Message,
}

mod api;
mod models;
mod store;

use std::sync::Mutex;

lazy_static! {
    static ref STORAGE: Mutex<store::MemoryStore> = Mutex::new(store::MemoryStore::new());
}

impl Backend for MyBackend {
    type S = MySession;

    fn new_session(&self) -> Result<MySession> {
        Ok(MySession {
            ..Default::default()
        })
    }
}

#[async_trait]
impl Session for MySession {
    fn auth_plain(&mut self, _username: &str, _password: &str) -> Result<()> {
        Ok(())
    }

    async fn mail(&mut self, from: &str, _: &MailOptions) -> Result<()> {
        println!("mail from: {}", from);
        self.message.from = from.to_owned();
        Ok(())
    }

    async fn rcpt(&mut self, to: &str) -> Result<()> {
        println!("rcpt to: {}", to);
        self.message.to = to.to_owned();
        Ok(())
    }

    async fn data<R: AsyncRead + Send + Unpin>(&mut self, r: R) -> Result<()> {
        // print whole message
        let mut data = Vec::new();
        let mut reader = io::BufReader::new(r);
        reader.read_to_end(&mut data).await?;
        println!("data: {}", String::from_utf8_lossy(&data));
        self.message.data = String::from_utf8_lossy(&data).to_string();

        STORAGE.lock().unwrap().add(self.message.clone());
        // STORAGE.add(self.message);

        Ok(())
    }

    fn reset(&mut self) {}

    fn logout(&mut self) -> Result<()> {
        Ok(())
    }
}

async fn serve_smtp_server() {
    let be = MyBackend;

    let mut s = Server::new(be);

    s.addr = "127.0.0.1:2525".to_string();
    s.domain = "mailtutan".to_string();
    s.read_timeout = std::time::Duration::from_secs(10);
    s.write_timeout = std::time::Duration::from_secs(10);
    s.max_message_bytes = 10 * 1024 * 1024;
    s.max_recipients = 50;
    s.max_line_length = 1000;
    s.allow_insecure_auth = true;

    println!("Starting server on {}", s.addr);
    s.listen_and_serve().await.unwrap();
}

#[tokio::main]
async fn main() {
    let runtime = Builder::new_multi_thread().enable_all().build().unwrap();

    let mut tasks = vec![];

    tasks.push(runtime.spawn(api::serve()));
    tasks.push(runtime.spawn(serve_smtp_server()));

    for task in tasks {
        task.await.unwrap();
    }
}
