use axum::{response::Html, routing::get, Router};

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub async fn serve() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(handler));

    println!("listening on 0.0.0.0:3000");

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
