use axum::{
    extract::State,
    http::{self},
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, User},
    services::{Storage, UserStore, UserStoreError},
};

pub async fn signup_handler<T>(
    State(app_state): State<AppState<T>>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError>
where
    T: UserStore,
{
    let email = request.email;
    let password = request.password;

    if email.trim().is_empty()
        || password.trim().is_empty()
        || password.len() < 8
        || !email.contains('@')
    {
        return Err(AuthAPIError::InvalidCredentials);
    }

    let user = User::new(email, password, request.requires_2fa);

    let mut user_store = app_state.user_store.write().await;

    match user_store.insert(user.email.clone(), user) {
        Ok(_) => {
            let response = Json(SignupResponse {
                message: "User created successfully!".to_string(),
            });
            Ok((http::StatusCode::CREATED, response))
        }
        Err(e) => {
            if let UserStoreError::UserAlreadyExists = e {
                return Err(AuthAPIError::UserAlreadyExists);
            } else {
                return Err(AuthAPIError::UnexpectedError);
            }
        }
    }
}

#[derive(Serialize)]
pub struct SignupResponse {
    pub message: String,
}

#[derive(serde::Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}
