use crate::APP;
use axum::{headers::HeaderMapExt, response::IntoResponse};

use axum::{
    headers::authorization::{Authorization, Basic},
    http::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
pub async fn basic<B>(request: Request<B>, next: Next<B>) -> Response {
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
    res
}
