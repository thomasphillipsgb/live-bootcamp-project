use axum::{http, response::IntoResponse, Json};
use serde_json::json;

use crate::utils::auth::validate_token;

#[derive(serde::Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

pub async fn verify_token_handler(Json(payload): Json<VerifyTokenRequest>) -> impl IntoResponse {
    let token = payload.token;
    if token.trim().is_empty() {
        return (
            http::StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "Malformed input"})),
        );
    }

    if validate_token(&token).await.is_ok() {
        (
            http::StatusCode::OK,
            Json(json!({"message": "Token is valid"})),
        )
    } else {
        (
            http::StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid token"})),
        )
    }
}
