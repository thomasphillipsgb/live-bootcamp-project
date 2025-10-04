use axum::{http, response::IntoResponse};

pub async fn signup_handler() -> impl IntoResponse {
    http::StatusCode::OK
}

