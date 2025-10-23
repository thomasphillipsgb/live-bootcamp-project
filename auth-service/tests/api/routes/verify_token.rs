use auth_service::{domain::models::Email, utils::auth::generate_auth_cookie};
use reqwest::Url;

use crate::helpers::TestApp;

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let response = app.post_verify_token(&"invalid_token").await;

    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    let cookie =
        generate_auth_cookie(&Email::new("email@example.com".to_string()).unwrap()).unwrap();

    // add valid cookie
    app.cookie_jar.add_cookie_str(
        &cookie.to_string(),
        &Url::parse("http://127.0.0.1").expect("Failed to parse URL"),
    );

    let response = app
        .post_verify_token(&serde_json::json!({
            "token": &cookie.value().to_string()
        }))
        .await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    let response = app
        .post_verify_token(&serde_json::json!({
            "token": "invalid.token.here"
        }))
        .await;

    assert_eq!(response.status().as_u16(), 401);
}
