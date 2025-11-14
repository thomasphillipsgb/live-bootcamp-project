use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use color_eyre::eyre::Context;
use tracing::instrument;

use crate::{
    app_state::AppState,
    domain::{models::Email, AuthAPIError, EmailClient},
    services::{BannedTokenStore, TwoFACodeStore, UserStore},
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

#[instrument(skip_all)]
pub async fn logout_handler<T, U, V, W>(
    jar: CookieJar,
    state: State<AppState<T, U, V, W>>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError>
where
    T: UserStore + Send + Sync,
    U: BannedTokenStore + Send + Sync,
    V: TwoFACodeStore,
    W: EmailClient,
{
    let cookie = jar.get(JWT_COOKIE_NAME).ok_or(AuthAPIError::MissingToken)?;
    let token = cookie.value().to_owned();

    validate_token(&token, &*state.banned_token_store.read().await)
        .await
        .map_err(|_| AuthAPIError::InvalidToken)?;

    let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));
    let mut banned_token_store = state.banned_token_store.write().await;
    banned_token_store
        .ban_token(&token)
        .await
        .wrap_err("Failed to ban token")
        .map_err(AuthAPIError::UnexpectedError)?;

    Ok((jar, StatusCode::OK))
}
