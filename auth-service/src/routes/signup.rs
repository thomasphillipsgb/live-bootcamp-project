use axum::{
    extract::State,
    http::{self},
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use crate::{
    app_state::AppState,
    domain::{
        models::{Email, Password},
        AuthAPIError, EmailClient, User,
    },
    services::{BannedTokenStore, TwoFACodeStore, UserStore, UserStoreError},
};

pub async fn signup_handler<T, U, V, W>(
    State(app_state): State<AppState<T, U, V, W>>,
    Json(request): Json<SignupRequest>,
) -> Result<impl IntoResponse, AuthAPIError>
where
    T: UserStore,
    U: BannedTokenStore,
    V: TwoFACodeStore,
    W: EmailClient,
{
    let email = request.email;
    let password = request.password;

    let (email, password) = match (Email::new(email), Password::new(password)) {
        (Ok(email), Ok(password)) => (email, password),
        _ => return Err(AuthAPIError::InvalidCredentials),
    };

    let user = User::new(email, password, request.requires_2fa);

    let mut user_store = app_state.user_store.write().await;

    match user_store.insert(user).await {
        Ok(_) => {
            let response = Json(SignupResponse {
                message: "User created successfully!".to_string(),
            });
            Ok((http::StatusCode::CREATED, response))
        }
        Err(e) => {
            if let UserStoreError::UserAlreadyExists = e {
                Err(AuthAPIError::UserAlreadyExists)
            } else {
                Err(AuthAPIError::UnexpectedError)
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
