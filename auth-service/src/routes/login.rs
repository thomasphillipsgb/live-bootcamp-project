use axum::{extract::State, http, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};

use crate::{
    app_state::AppState,
    domain::{
        models::{Email, Password},
        AuthAPIError,
    },
    services::{BannedTokenStore, TwoFACodeStore, UserStore},
    utils::auth::generate_auth_cookie,
};

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login_handler<T, U, V>(
    State(state): State<AppState<T, U, V>>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>)
where
    T: UserStore + Send + Sync,
    U: BannedTokenStore,
    V: TwoFACodeStore,
{
    let email = request.email;
    let password = request.password;

    let (email, password) = match (Email::new(email), Password::new(password)) {
        (Ok(email), Ok(password)) => (email, password),
        _ => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = &state.user_store.read().await;
    if let Ok(_) = user_store.validate(&email, password.as_ref()).await {
        let user = user_store.get(&email).await.unwrap();
        match user.requires_2fa {
            true => handle_2fa(jar).await,
            false => handle_no_2fa(&user.email, jar).await,
        }
    } else {
        (jar, Err(AuthAPIError::IncorrectCredentials))
    }
}

async fn handle_2fa(
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(http::StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    (
        jar,
        Ok((
            http::StatusCode::PARTIAL_CONTENT,
            Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
                message: "2FA required".to_string(),
                login_attempt_id: "123456".to_string(),
            })),
        )),
    )
}

// New!
async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(http::StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let auth_cookie = generate_auth_cookie(&email);

    let jar = jar.add(auth_cookie.unwrap());

    (
        jar,
        Ok((http::StatusCode::OK, Json(LoginResponse::RegularAuth))),
    )
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

// If a user requires 2FA, this JSON body should be returned!
#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}
