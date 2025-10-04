use axum::{http, response::IntoResponse};

pub async fn verify_token_handler() -> impl IntoResponse {
    http::StatusCode::OK
}