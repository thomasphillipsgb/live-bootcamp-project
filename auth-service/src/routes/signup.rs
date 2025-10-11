use axum::{
    extract::State, http::{self}, response::IntoResponse, Json
};
use serde::Serialize;

use crate::{app_state::AppState, domain::User, services::Storage};

pub async fn signup_handler(
    State(app_state): State<AppState>,
    Json(request): Json<SignupRequest>,
) -> impl IntoResponse {
    // Create a new `User` instance using data in the `request`
    let user = User::new(request.email, request.password, request.requires_2fa);

    let mut user_store = app_state.user_store.write().await;

    // TODO: Add `user` to the `user_store`. Simply unwrap the returned `Result` enum type for now.
    user_store.insert(user.email.clone(), user).unwrap();

    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });

    (http::StatusCode::CREATED, response)
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
