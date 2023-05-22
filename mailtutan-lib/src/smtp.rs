use crate::models::Message;
use crate::models::MessageEvent;
use crate::storage::Connection;
use mailin_embedded::{Handler, Server, SslConfig};
use std::io;
use std::sync::Arc;

#[derive(Clone)]
struct MyHandler {
    pub data: Vec<u8>,
    pub conn: Arc<Connection>,
}

impl Handler for MyHandler {
    fn data(&mut self, buf: &[u8]) -> io::Result<()> {
        self.data.append(&mut buf.to_owned());

        Ok(())
    }

    fn data_end(&mut self) -> mailin_embedded::Response {
        let message = Message::from(&self.data);

        let event = MessageEvent {
            event_type: "add".to_owned(),
            message: message.clone(),
        };

        self.conn.storage.lock().unwrap().add(message);

        self.conn
            .ws_sender
            .clone()
            .send(serde_json::to_string(&event).unwrap())
            .ok();

        mailin_embedded::response::OK
    }
}

pub async fn serve(conn: Arc<Connection>) {
    let handler = MyHandler {
        data: vec![],
        conn: conn.clone(),
    };
    let mut server = Server::new(handler);

    server
        .with_name("example.com")
        .with_ssl(SslConfig::None)
        .unwrap()
        .with_addr("127.0.0.1:1025")
        .unwrap();

    server.serve().unwrap();
}
