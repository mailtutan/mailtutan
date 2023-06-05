use std::convert::Infallible;

use axum::{
    body::{Bytes, Full},
    headers::HeaderMapExt,
    response::IntoResponse,
    routing::delete,
    routing::get,
    Router,
};

use crate::APP;

mod assets;
mod messages;
mod version;
mod websocket;

use axum::{
    extract::TypedHeader,
    headers::authorization::{Authorization, Bearer},
    http::Request,
    http::StatusCode,
    middleware::{self, Next},
    response::Response,
};
// use axum::response::IntoResponse;
// async fn auth<B>(
//     TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
//     request: Request<B>,
//     next: Next<B>,
// ) -> Result<Response, StatusCode> {
//     if token_is_valid(auth.token()) {
//         let response = next.run(request).await;
//         Ok(response)
//     } else {
//         Err(StatusCode::UNAUTHORIZED)
//     }
// }
//
// fn token_is_valid(token: &str) -> bool {
//     false
// }

async fn auth<B>(request: Request<B>, next: Next<B>) -> Response {
    let b = request.headers().typed_get::<Authorization<Bearer>>();

    if b.is_none() {
        dbg!("it is none");
        let mut res = (StatusCode::UNAUTHORIZED, "nothing").into_response();
        res.headers_mut().insert(
            "WWW-Authenticate",
            "Basic realm=\"Mailtutan\"".parse().unwrap(),
        );
        return res;
    }

    b.unwrap().0.username();
    // dbg!(&b.usern;

    let response = next.run(request).await;

    return response;
    // do something with `response`...
}
pub async fn serve() {
    let app = Router::new()
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
        .route(
            "/api/messages/:id/parts/:cid",
            get(messages::download_attachment),
        )
        .route("/api/messages", delete(messages::delete_all))
        .route("/api/version", get(version::show))
        .route_layer(axum::middleware::from_fn(auth));

    let uri = APP.get().unwrap().lock().unwrap().get_api_uri();

    println!("listening on http://{}", uri);

    axum::Server::bind(&uri.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
