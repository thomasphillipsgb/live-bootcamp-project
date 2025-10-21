use axum::{http, response::IntoResponse, Json};

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login_handler(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    http::StatusCode::OK
}
