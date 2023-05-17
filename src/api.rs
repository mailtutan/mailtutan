use axum::{routing::get, Router};

mod assets;
mod messages;
mod version;
mod websocket;

pub async fn serve() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(assets::index_html))
        .route("/ws", get(websocket::websocket_handler))
        .route("/assets/:name", get(assets::show))
        .route("/api/messages", get(messages::index))
        .route("/api/messages/:id/source", get(messages::show_source))
        .route("/api/messages/:id/json", get(messages::show_json))
        .route("/api/messages/delete_all", get(messages::delete_all))
        .route("/api/version", get(version::show));

    println!("listening on http://0.0.0.0:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
