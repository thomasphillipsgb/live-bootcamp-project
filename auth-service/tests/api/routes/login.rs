use auth_service::utils::constants::JWT_COOKIE_NAME;

use crate::helpers::TestApp;

#[tokio::test]
async fn login_returns_422_if_malformed_request() {
    let app = TestApp::new().await;

    let response = app.post_login(&serde_json::json!({"invalid": "data"})).await;

    assert_eq!(response.status(), 422);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new().await;

    let response = app.post_login(&serde_json::json!({
        "email": "invalid-email",
        "password": "short"
    })).await;

    assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new().await;

    let response = app.post_login(&serde_json::json!({
        "email": "user@example.com",
        "password": "wrong-password"
    })).await;

    assert_eq!(response.status(), 401);
}

#[tokio::test]
async fn should_return_200_if_valid_credentials_and_2fa_disabled() {
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
}