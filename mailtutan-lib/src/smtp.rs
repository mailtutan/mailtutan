use crate::models::Message;
use crate::models::MessageEvent;
use crate::AppState;
use anyhow::Result;
use mailin::AuthMechanism;
use mailin_embedded::response::{self, Response};
use mailin_embedded::{Handler, SslConfig};
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

pub struct Server(mailin_embedded::Server<MyHandler>);

impl Server {
    pub async fn serve(self) -> Result<()> {
        self.0.serve().unwrap();

        Ok(())
    }
}
pub struct Builder {
    state: Option<Arc<AppState>>,
    ssl_config: SslConfig,
    socket: Option<SocketAddr>,
    auth: bool,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            state: None,
            ssl_config: SslConfig::None,
            socket: None,
            auth: false,
        }
    }

    pub fn with_state(mut self, state: Arc<AppState>) -> Self {
        self.state = Some(state);
        self
    }

    pub fn with_auth(mut self, value: bool) -> Self {
        self.auth = value;
        self
    }

    pub fn with_ssl(mut self, cert_path: Option<String>, key_path: Option<String>) -> Self {
        if let (Some(cert_path), Some(key_path)) = (cert_path, key_path) {
            self.ssl_config = SslConfig::SelfSigned {
                cert_path,
                key_path,
            };
        }
        self
    }

    pub fn bind(mut self, socket: SocketAddr) -> Self {
        self.socket = Some(socket);
        self
    }

    pub fn build(self) -> Server {
        let handler = MyHandler {
            data: vec![],
            state: self.state.unwrap(),
        };
        let mut server = mailin_embedded::Server::new(handler);

        server
            .with_ssl(self.ssl_config)
            .expect("SslConfig error")
            .with_addr(self.socket.unwrap())
            .unwrap();

        if self.auth {
            server.with_auth(AuthMechanism::Plain);
        }

        println!("listening on smtp://{}", self.socket.unwrap());

        Server(server)
    }
}

#[derive(Clone)]
pub struct MyHandler {
    pub data: Vec<u8>,
    pub state: Arc<AppState>,
}

impl Handler for MyHandler {
    fn data(&mut self, buf: &[u8]) -> io::Result<()> {
        self.data.append(&mut buf.to_owned());

        Ok(())
    }

    fn mail(&mut self, _ip: std::net::IpAddr, _domain: &str, _from: &str) -> Response {
        self.data.clear();

        mailin_embedded::response::OK
    }

    fn data_end(&mut self) -> mailin_embedded::Response {
        let message = Message::from(&self.data).unwrap();

        let msg = self.state.storage.write().unwrap().add(message);

        let event = MessageEvent {
            event_type: "add".to_owned(),
            message: msg,
        };

        self.state
            .channel
            .clone()
            .send(serde_json::to_string(&event).unwrap())
            .ok();

        mailin_embedded::response::OK
    }

    fn auth_plain(
        &mut self,
        _authorization_id: &str,
        authentication_id: &str,
        password: &str,
    ) -> Response {
        if authentication_id == self.state.smtp_auth_username.as_ref().unwrap()
            && password == self.state.smtp_auth_password.as_ref().unwrap()
        {
            response::AUTH_OK
        } else {
            response::INVALID_CREDENTIALS
        }
    }
}
