use axum::{http, response::IntoResponse};

pub async fn logout_handler() -> impl IntoResponse {
    http::StatusCode::OK
}

