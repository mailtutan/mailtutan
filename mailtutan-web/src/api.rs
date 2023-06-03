use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;

pub fn delete_messages() {
    spawn_local(async move {
        Request::delete("/api/messages").send().await.unwrap();
    });
}
