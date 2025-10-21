use axum::{http, response::IntoResponse, Json};

use crate::domain::{
    models::{Email, Password},
    AuthAPIError,
};

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login_handler(
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError> {
    let email = request.email;
    let password = request.password;

    let (email, password) = match (Email::new(email), Password::new(password)) {
        (Ok(email), Ok(password)) => (email, password),
        _ => return Err(AuthAPIError::InvalidCredentials),
    };

    Ok(http::StatusCode::OK)
}
