use axum::{extract::State, http, response::IntoResponse, Json};

use crate::{app_state::AppState, domain::{
    models::{Email, Password},
    AuthAPIError,
}, services::UserStore};

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login_handler<T>(
    State(state): State<AppState<T>>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, AuthAPIError>
where
    T: UserStore,
    {
    let email = request.email;
    let password = request.password;

    let (email, password) = match (Email::new(email), Password::new(password)) {
        (Ok(email), Ok(password)) => (email, password),
        _ => return Err(AuthAPIError::InvalidCredentials),
    };

    let user_store = &state.user_store.read().await;
    if let Ok(_) = user_store.validate(&email, password.as_ref()) {
        Ok(http::StatusCode::OK)
    } else {
        Err(AuthAPIError::IncorrectCredentials)
    }
}
