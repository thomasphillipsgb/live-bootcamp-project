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