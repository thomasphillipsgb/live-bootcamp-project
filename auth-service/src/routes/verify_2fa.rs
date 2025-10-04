use axum::{http, response::IntoResponse};

pub async fn verify_2fa_handler() -> impl IntoResponse {
    http::StatusCode::OK
}

