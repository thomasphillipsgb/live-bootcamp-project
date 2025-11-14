use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use color_eyre::eyre::Context;
use color_eyre::eyre::Result;
use tracing::instrument;

use crate::{
    app_state::AppState,
    domain::{models::Email, AuthAPIError, EmailClient},
    services::{BannedTokenStore, LoginAttemptId, TwoFACode, TwoFACodeStore, UserStore},
    utils::auth::generate_auth_cookie,
};

#[instrument(skip_all)]
pub async fn verify_2fa_handler<T, U, V, W>(
    jar: CookieJar,
    State(state): State<AppState<T, U, V, W>>,
    Json(request): Json<Verify2FARequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>)
where
    T: UserStore + Send + Sync,
    U: BannedTokenStore,
    V: TwoFACodeStore,
    W: EmailClient,
{
    match (
        Email::new(request.email),
        LoginAttemptId::new(request.login_attempt_id),
        TwoFACode::new(request.two_fa_code),
    ) {
        (Ok(email), Ok(login_attempt_id), Ok(_two_fa_code)) => {
            let mut two_fa_code_store = state.two_fa_code_store.write().await;

            // Call `two_fa_code_store.get_code`. If the call fails
            // return a `AuthAPIError::IncorrectCredentials`.
            let code_tuple = two_fa_code_store.get_code(&email).await;
            match code_tuple {
                Err(_) => (jar, Err(AuthAPIError::IncorrectCredentials.into())),
                Ok((attempt, code)) => {
                    if attempt != login_attempt_id || code.as_ref() != _two_fa_code.as_ref() {
                        return (jar, Err(AuthAPIError::IncorrectCredentials.into()));
                    }

                    match generate_auth_cookie(&email).map_err(AuthAPIError::UnexpectedError) {
                        Err(e) => return (jar, Err(e.into())),
                        Ok(auth_cookie) => {
                            let jar = jar.add(auth_cookie);

                            return match two_fa_code_store.remove_code(&email).await {
                                Err(e) => {
                                    (jar, Err(AuthAPIError::UnexpectedError(e.into()).into()))
                                }
                                Ok(_) => (jar, Ok(StatusCode::OK.into_response())),
                            };
                        }
                    }
                }
            }
        }
        _ => (jar, Ok(StatusCode::BAD_REQUEST.into_response())),
    }
}

#[derive(serde::Deserialize)]
pub struct Verify2FARequest {
    pub email: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
    #[serde(rename = "2FACode")]
    pub two_fa_code: String,
}
