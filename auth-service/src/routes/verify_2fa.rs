use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::{
    domain::models::Email,
    services::{LoginAttemptId, TwoFACode},
};

pub async fn verify_2fa_handler(Json(request): Json<Verify2FARequest>) -> impl IntoResponse {
    match (
        Email::new(request.email),
        LoginAttemptId::new(request.login_attempt_id),
        TwoFACode::new(request.two_fa_code),
    ) {
        (Ok(_email), Ok(_login_attempt_id), Ok(_two_fa_code)) => StatusCode::OK.into_response(),
        _ => StatusCode::BAD_REQUEST.into_response(),
    }
}

#[derive(serde::Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    pub login_attempt_id: String,
    pub two_fa_code: String,
}
