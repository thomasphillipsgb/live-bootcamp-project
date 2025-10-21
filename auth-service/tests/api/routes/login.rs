use crate::helpers::TestApp;

#[tokio::test]
async fn login_returns_422_if_malformed_request() {
    let app = TestApp::new().await;

    let response = app.post_login(&serde_json::json!({"invalid": "data"})).await;

    assert_eq!(response.status(), 422);
}
