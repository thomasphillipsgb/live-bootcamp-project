use axum::{extract::State, http, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use color_eyre::eyre::eyre;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    app_state::AppState,
    domain::{
        models::{Email, Password},
        AuthAPIError, EmailClient,
    },
    services::{BannedTokenStore, LoginAttemptId, TwoFACode, TwoFACodeStore, UserStore},
    utils::auth::generate_auth_cookie,
};

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: SecretString,
}

#[instrument(skip_all)]
pub async fn login_handler<T, U, V, W>(
    State(state): State<AppState<T, U, V, W>>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>)
where
    T: UserStore + Send + Sync,
    U: BannedTokenStore,
    V: TwoFACodeStore,
    W: EmailClient,
{
    let email = request.email;
    let password = request.password;

    let (email, password) = match (Email::new(email.into()), Password::new(password)) {
        (Ok(email), Ok(password)) => (email, password),
        _ => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = &state.user_store.read().await;
    if let Ok(_) = user_store.validate(&email, password.as_ref()).await {
        let user = user_store.get(&email).await.unwrap();
        match user.requires_2fa {
            true => handle_2fa(&email, &state, jar).await,
            false => handle_no_2fa(&user.email, jar).await,
        }
    } else {
        (jar, Err(AuthAPIError::IncorrectCredentials))
    }
}

#[instrument(skip_all)]
async fn handle_2fa<T, U, V, W>(
    email: &Email,
    state: &AppState<T, U, V, W>,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(http::StatusCode, Json<LoginResponse>), AuthAPIError>,
)
where
    T: UserStore + Send + Sync,
    U: BannedTokenStore,
    V: TwoFACodeStore,
    W: EmailClient,
{
    // First, we must generate a new random login attempt ID and 2FA code
    let login_attempt_id = LoginAttemptId::default();
    let code = TwoFACode::default();

    let two_fa_store = &mut state.two_fa_code_store.write().await;
    let add_result = two_fa_store
        .add_code(email.clone(), login_attempt_id.clone(), code.clone())
        .await;

    if let Err(e) = add_result {
        return (jar, Err(AuthAPIError::UnexpectedError(e.into())));
    }

    let email_client = &state.email_client.read().await;

    if let Err(e) = email_client
        .send_email(
            email,
            "Your 2FA Code",
            &format!("Your 2FA code is: {}", code.as_ref()),
        )
        .await
    {
        return (jar, Err(AuthAPIError::UnexpectedError(eyre!(e))));
    }

    let response = Json(LoginResponse::TwoFactorAuth(TwoFactorAuthResponse {
        message: "2FA required".to_owned(),
        login_attempt_id: login_attempt_id.as_ref().to_owned(),
    }));
    return (jar, Ok((http::StatusCode::PARTIAL_CONTENT, response)));
}

#[instrument(skip_all)]
async fn handle_no_2fa(
    email: &Email,
    jar: CookieJar,
) -> (
    CookieJar,
    Result<(http::StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let jar = match generate_auth_cookie(&email) {
        Ok(cookie) => jar.add(cookie),
        Err(e) => return (jar, Err(AuthAPIError::UnexpectedError(e))),
    };
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
