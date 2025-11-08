use auth_service::domain::models::Email;
use auth_service::services::BannedTokenStore;
use auth_service::utils::{auth::generate_auth_cookie, constants::JWT_COOKIE_NAME};
use reqwest::Url;

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_400_if_jwt_cookie_missing() {
    let app = TestApp::new().await;

    let response = app.post_logout().await;
    assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // add invalid cookie
    app.cookie_jar.add_cookie_str(
        &format!(
            "{}=invalid; HttpOnly; SameSite=Lax; Secure; Path=/",
            JWT_COOKIE_NAME
        ),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );
    let response = app.post_logout().await;

    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_jwt_cookie() {
    let app = TestApp::new().await;
    let random_email = "user".to_string() + &uuid::Uuid::new_v4().to_string() + "@example.com";

    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": false
    });

    let response = app.post_signup(&signup_body).await;

    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });

    let response = app.post_login(&login_body).await;

    assert_eq!(response.status().as_u16(), 200);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found");

    assert!(!auth_cookie.value().is_empty());

    let response = app.post_logout().await;
    let banned_token_store = app.banned_token_store.read().await;

    assert!(
        banned_token_store
            .is_token_banned(auth_cookie.value())
            .await
    );

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn should_return_400_if_logout_called_twice_in_a_row() {
    let app = TestApp::new().await;

    // add valid cookie
    app.cookie_jar.add_cookie_str(
        &generate_auth_cookie(&Email::new("email@example.com".to_string()).unwrap())
            .unwrap()
            .to_string(),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );
    let response = app.post_logout().await;

    assert_eq!(response.status(), 200);

    let response = app.post_logout().await;

    assert_eq!(response.status(), 400);
}
