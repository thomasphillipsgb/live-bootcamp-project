use auth_service::{
    routes::TwoFactorAuthResponse, services::TwoFACodeStore, utils::constants::JWT_COOKIE_NAME,
};

use crate::helpers::TestApp;

#[tokio::test]
async fn login_returns_422_if_malformed_request() {
    let app = TestApp::new().await;

    let response = app
        .post_login(&serde_json::json!({"invalid": "data"}))
        .await;

    assert_eq!(response.status(), 422);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    // Call the log-in route with invalid credentials and assert that a
    // 400 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new().await;

    let response = app
        .post_login(&serde_json::json!({
            "email": "invalid-email",
            "password": "short"
        }))
        .await;

    assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    // Call the log-in route with incorrect credentials and assert
    // that a 401 HTTP status code is returned along with the appropriate error message.
    let app = TestApp::new().await;

    let response = app
        .post_login(&serde_json::json!({
            "email": "user@example.com",
            "password": "wrong-password"
        }))
        .await;

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

#[tokio::test]
async fn should_return_206_if_valid_credentials_and_2fa_enabled() {
    let app = TestApp::new().await;

    let random_email = "user".to_string() + &uuid::Uuid::new_v4().to_string() + "@example.com";
    let signup_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
        "requires2FA": true
    });
    let response = app.post_signup(&signup_body).await;
    assert_eq!(response.status().as_u16(), 201);

    let login_body = serde_json::json!({
        "email": random_email,
        "password": "password123",
    });
    let response = app.post_login(&login_body).await;
    assert_eq!(response.status().as_u16(), 206);

    let response = response.json::<TwoFactorAuthResponse>().await.unwrap();

    assert_eq!(response.message, "2FA required".to_owned());

    // TODO: assert that `json_body.login_attempt_id` is stored inside `app.two_fa_code_store`
    let login_attempt_id = response.login_attempt_id;

    let store = app.two_fa_code_store.read().await;
    let result = store
        .get_code(&auth_service::domain::models::Email::new(random_email).unwrap())
        .await;
    assert!(result.is_ok());

    let (stored_login_attempt_id, _) = result.unwrap();
    assert_eq!(stored_login_attempt_id.as_ref(), login_attempt_id.as_str());
}
