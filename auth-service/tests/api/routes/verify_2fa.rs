use auth_service::{
    domain::{
        models::{Email, Password},
        User,
    },
    services::UserStore,
};

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let response = app.post_verify_2fa(&()).await;

    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let response = app
        .post_verify_2fa(&serde_json::json!({
            "email": "userm",
            "login_attempt_id": "string",
            "two_fa_code": "string"
        }))
        .await;

    assert_eq!(response.status().as_u16(), 400);
}

#[tokio::test]
async fn should_return_401_if_incorrect_credentials() {
    let app = TestApp::new().await;
    app.user_store
        .write()
        .await
        .insert(User::new(
            Email::new("user@example.com".to_string()).unwrap(),
            Password::new("correct_password".to_string()).unwrap(),
            true,
        ))
        .await
        .unwrap();

    let login_response = app
        .post_login(&serde_json::json!({
            "email": "user@example.com",
            "password": "correct_password",
            "requires2fa": true
        }))
        .await;

    assert_eq!(login_response.status().as_u16(), 206);

    let response = app
        .post_verify_2fa(&serde_json::json!({
            "email": "ur@example.com",
            "login_attempt_id": uuid::Uuid::new_v4().to_string(),
            "two_fa_code": "123456"
        }))
        .await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn should_return_401_if_old_code() {
    // Call login twice. Then, attempt to call verify-fa with the 2FA code from the first login requet. This should fail.
    let app = TestApp::new().await;
    app.user_store
        .write()
        .await
        .insert(User::new(
            Email::new("user@example.com".to_string()).unwrap(),
            Password::new("correct_password".to_string()).unwrap(),
            true,
        ))
        .await
        .unwrap();

    let login_response = app
        .post_login(&serde_json::json!({
            "email": "user@example.com",
            "password": "correct_password",
            "requires2fa": true
        }))
        .await;
    assert_eq!(login_response.status().as_u16(), 206);

    let login_response = app
        .post_login(&serde_json::json!({
            "email": "user@example.com",
            "password": "correct_password",
            "requires2fa": true
        }))
        .await;

    assert_eq!(login_response.status().as_u16(), 206);

    let response = app
        .post_verify_2fa(&serde_json::json!({
            "email": "user@example.com",
            "login_attempt_id": uuid::Uuid::new_v4().to_string(),
            "two_fa_code": "123456"
        }))
        .await;
    assert_eq!(response.status().as_u16(), 401);
}
