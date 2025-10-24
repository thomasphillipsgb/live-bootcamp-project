use axum::{extract::State, http, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;

use crate::{
    app_state::AppState,
    domain::{
        models::{Email, Password},
        AuthAPIError,
    },
    services::{BannedTokenStore, UserStore},
    utils::auth::generate_auth_cookie,
};

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login_handler<T, U>(
    State(state): State<AppState<T, U>>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>)
where
    T: UserStore,
    U: BannedTokenStore,
{
    let email = request.email;
    let password = request.password;

    let (email, password) = match (Email::new(email), Password::new(password)) {
        (Ok(email), Ok(password)) => (email, password),
        _ => return (jar, Err(AuthAPIError::InvalidCredentials)),
    };

    let user_store = &state.user_store.read().await;
    if let Ok(_) = user_store.validate(&email, password.as_ref()) {
        let auth_cookie = generate_auth_cookie(&email);

        let jar = jar.add(auth_cookie.unwrap());
        (jar, Ok(http::StatusCode::OK))
    } else {
        (jar, Err(AuthAPIError::IncorrectCredentials))
    }
}
