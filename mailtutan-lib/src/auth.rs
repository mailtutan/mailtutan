use axum::{headers::HeaderMapExt, response::IntoResponse};

use crate::AppState;
use axum::extract::State;
use axum::{
    headers::authorization::{Authorization, Basic},
    http::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

pub async fn basic<B>(
    State(state): State<Arc<AppState>>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    if let Some(credential) = request.headers().typed_get::<Authorization<Basic>>() {
        if state.http_auth_username == credential.0.username()
            && state.http_auth_password == credential.0.password()
        {
            return next.run(request).await;
        }
    }

    let mut res = (StatusCode::UNAUTHORIZED, "Authorization is required").into_response();
    res.headers_mut().insert(
        "WWW-Authenticate",
        "Basic realm=\"Mailtutan\"".parse().unwrap(),
    );
    res
}
