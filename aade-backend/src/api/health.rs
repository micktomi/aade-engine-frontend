use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;

pub async fn readiness() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "status": "ready" })))
}

pub async fn liveness() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "status": "alive" })))
}
