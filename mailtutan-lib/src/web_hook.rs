use crate::AppState;
use reqwest;
use std::sync::Arc;

pub struct Worker {
    state: Arc<AppState>,
}

impl Worker {
    pub fn new(state: Arc<AppState>) -> Self {
        Worker { state }
    }

    pub async fn serve(self) {
        let mut rx = self.state.channel.subscribe();

        while let Ok(msg) = rx.recv().await {
            if let Some(url) = &self.state.web_hook {
                let client = reqwest::Client::new();
                client.post(url).body(msg).send().await.ok();
            }
        }
    }
}
