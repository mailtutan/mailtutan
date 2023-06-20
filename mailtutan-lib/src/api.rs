use std::{net::SocketAddr, sync::Arc};

use anyhow::Ok;
use axum::{routing::delete, routing::get, Router};

use crate::{auth, AppState};

use anyhow::Result;

mod assets;
mod messages;
mod version;
mod websocket;

pub struct Builder {
    http_auth: bool,
    socket: Option<SocketAddr>,
    state: Option<Arc<AppState>>,
}

pub struct Server {
    router: Router,
    socket: SocketAddr,
}

impl Server {
    pub async fn serve(self) -> Result<()> {
        println!("listening on http://{}", self.socket.to_string());

        axum::Server::bind(&self.socket)
            .serve(self.router.into_make_service())
            .await?;

        Ok(())
    }
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            http_auth: false,
            socket: None,
            state: None,
        }
    }

    pub fn http_auth(mut self, value: bool) -> Self {
        self.http_auth = value;
        self
    }

    pub fn bind(mut self, socket: SocketAddr) -> Self {
        self.socket = Some(socket);
        self
    }

    pub fn with_state(mut self, state: Arc<AppState>) -> Self {
        self.state = Some(state);
        self
    }

    pub fn build(self) -> Server {
        let state = self.state.unwrap();

        let router = Router::new()
            .route("/", get(assets::index_html))
            .route("/ws", get(websocket::websocket_handler))
            .route("/mailtutan-web_bg.wasm", get(assets::wasm))
            .route("/styles.css", get(assets::css))
            .route("/mailtutan-web.js", get(assets::js))
            .route("/api/messages", get(messages::index))
            .route("/api/messages/:id/source", get(messages::show_source))
            .route("/api/messages/:id/plain", get(messages::show_plain))
            .route("/api/messages/:id/html", get(messages::show_html))
            .route("/api/messages/:id/json", get(messages::show_json))
            .route("/api/messages/:id/eml", get(messages::show_eml))
            .route("/api/messages/:id", delete(messages::delete))
            .route(
                "/api/messages/:id/parts/:cid",
                get(messages::download_attachment),
            )
            .route("/api/messages", delete(messages::delete_all))
            .route("/api/version", get(version::show))
            .with_state(state.clone());

        let router = {
            if self.http_auth {
                router.route_layer(axum::middleware::from_fn_with_state(
                    state.clone(),
                    auth::basic,
                ))
            } else {
                router
            }
        };

        Server {
            router,
            socket: self.socket.unwrap(),
        }
    }
}
