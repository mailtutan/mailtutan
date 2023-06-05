use axum::{headers::HeaderMapExt, response::IntoResponse, routing::delete, routing::get, Router};

use crate::APP;

mod assets;
mod messages;
mod version;
mod websocket;

use axum::{
    headers::authorization::{Authorization, Basic},
    http::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};

async fn auth<B>(request: Request<B>, next: Next<B>) -> Response {
    if let Some(credential) = request.headers().typed_get::<Authorization<Basic>>() {
        if let Some(app) = APP.get() {
            let valid = {
                if let Ok(app) = app.lock() {
                    app.http_username == credential.0.username()
                        && app.http_password == credential.0.password()
                } else {
                    false
                }
            };

            if valid {
                let res = next.run(request).await;
                return res;
            }
        }
    }

    let mut res = (StatusCode::UNAUTHORIZED, "Authorization is required").into_response();
    res.headers_mut().insert(
        "WWW-Authenticate",
        "Basic realm=\"Mailtutan\"".parse().unwrap(),
    );
    return res;
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
        .route("/api/version", get(version::show));

    let app = {
        if APP.get().unwrap().lock().unwrap().http_auth {
            app.route_layer(axum::middleware::from_fn(auth))
        } else {
            app
        }
    };

    let uri = APP.get().unwrap().lock().unwrap().get_api_uri();

    println!("listening on http://{}", uri);

    axum::Server::bind(&uri.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
