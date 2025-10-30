use axum::{extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::{
    app_state::AppState,
    domain::{models::Email, AuthAPIError},
    services::{BannedTokenStore, UserStore},
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout_handler<T, U>(
    jar: CookieJar,
    state: State<AppState<T, U>>,
) -> Result<(CookieJar, impl IntoResponse), AuthAPIError>
where
    T: UserStore + Send + Sync,
    U: BannedTokenStore,
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
        .map_err(|_| AuthAPIError::UnexpectedError)?;

    Ok((jar, StatusCode::OK))
}
