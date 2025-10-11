use crate::helpers::{get_random_email, TestApp};

#[tokio::test]
async fn should_return_422_for_invalid_signup() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let test_cases = [
        serde_json::json!({ "password": "anotherPassword!", "requires2FA": false }),
        serde_json::json!({ "email": email, "requires2FA": false }),
        serde_json::json!({ "email": email, "password": "anotherPassword!" }),
        ];

    for case in test_cases {
        let response = app.post_signup(&case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for case: {}",
            case
        );
    }
}

#[tokio::test]
async fn should_return_201_for_valid_signup() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let body = serde_json::json!({
        "email": email,
        "password": "anotherPassword!",
        "requires2FA": false
    });

    let response = app.post_signup(&body).await;
    assert_eq!(response.status().as_u16(), 201);
}

#[tokio::test]
async fn should_return_409_for_duplicate_signup() {
    let app = TestApp::new().await;

    let email = get_random_email();
    let body = serde_json::json!({
        "email": email,
        "password": "anotherPassword!",
        "requires2FA": false
    });

    let response1 = app.post_signup(&body).await;
    assert_eq!(response1.status().as_u16(), 201);

    let response2 = app.post_signup(&body).await;
    assert_eq!(response2.status().as_u16(), 409);
}