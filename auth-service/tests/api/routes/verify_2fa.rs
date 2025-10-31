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
