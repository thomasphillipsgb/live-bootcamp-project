use axum::{http::{self}, response::IntoResponse, Json};

pub async fn signup_handler(Json(request): Json<SignupRequest>) -> impl IntoResponse {
    http::StatusCode::OK
}

#[derive(serde::Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}
