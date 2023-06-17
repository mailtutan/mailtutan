use axum::response::Json;
use serde::Serialize;

pub async fn show() -> Json<VersionInfo> {
    let vi = VersionInfo {
        version: env!("CARGO_PKG_VERSION"),
    };
    Json(vi)
}

#[derive(Debug, Serialize)]
pub struct VersionInfo {
    version: &'static str,
}
