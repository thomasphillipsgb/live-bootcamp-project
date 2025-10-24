use std::ops::Deref;

use axum::{extract::State, http, response::IntoResponse, Json};
use serde_json::json;

use crate::{
    app_state::AppState,
    services::{BannedTokenStore, UserStore},
    utils::auth::validate_token,
};

#[derive(serde::Deserialize)]
pub struct VerifyTokenRequest {
    pub token: String,
}

pub async fn verify_token_handler<T, U>(
    State(app_state): State<AppState<T, U>>,
    Json(payload): Json<VerifyTokenRequest>,
) -> impl IntoResponse
where
    T: UserStore,
    U: BannedTokenStore,
{
    let token = payload.token;
    if token.trim().is_empty() {
        return (
            http::StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({"error": "Malformed input"})),
        );
    }

    if validate_token(&token, &*app_state.banned_token_store.read().await)
        .await
        .is_ok()
    {
        (
            http::StatusCode::OK,
            Json(json!({"message": "Token is valid"})),
        )
    } else {
        (
            http::StatusCode::UNAUTHORIZED,
            Json(json!({"error": "Invalid token"})),
        )
    }
}
