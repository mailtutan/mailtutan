use crate::models::Message;
use crate::models::MessageEvent;
use crate::APP;
use mailin_embedded::{Handler, Server, SslConfig};
use std::io;

#[derive(Clone)]
struct MyHandler {
    pub data: Vec<u8>,
}

impl Handler for MyHandler {
    fn data(&mut self, buf: &[u8]) -> io::Result<()> {
        self.data.append(&mut buf.to_owned());

        Ok(())
    }

    fn data_end(&mut self) -> mailin_embedded::Response {
        let message = Message::from(&self.data);

        let msg = APP
            .get()
            .expect("get app")
            .lock()
            .expect("get lock")
            .storage
            .add(message);

        let event = MessageEvent {
            event_type: "add".to_owned(),
            message: msg,
        };

        APP.get()
            .expect("get app")
            .lock()
            .expect("get lock")
            .ws_sender
            .clone()
            .send(serde_json::to_string(&event).unwrap())
            .ok();

        mailin_embedded::response::OK
    }
}

pub async fn serve() {
    let handler = MyHandler { data: vec![] };
    let mut server = Server::new(handler);
    let uri = APP.get().unwrap().lock().unwrap().get_smtp_uri();

    let ssl: SslConfig = {
        let app = APP.get().unwrap().lock().unwrap();

        if let (Some(cert_path), Some(key_path)) =
            (app.smtp_cert_path.clone(), app.smtp_key_path.clone())
        {
            SslConfig::SelfSigned {
                cert_path: cert_path,
                key_path: key_path,
            }
        } else {
            SslConfig::None
        }
    };

    server
        .with_ssl(ssl)
        .expect("SslConfig error")
        .with_addr(&uri)
        .unwrap();

    println!("listening on smtp://{}", &uri);

    server.serve().unwrap();
}
