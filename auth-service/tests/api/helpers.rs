use std::sync::Arc;

use auth_service::{services::BannedTokenStore, utils::constants::test, Application};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<reqwest::cookie::Jar>,
    pub http_client: reqwest::Client,
    pub banned_token_store: Arc<tokio::sync::RwLock<dyn BannedTokenStore>>,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = Arc::new(tokio::sync::RwLock::new(
            auth_service::services::hashmap_user_store::HashMapUserStore::new(),
        ));
        let banned_token_store = Arc::new(tokio::sync::RwLock::new(
            auth_service::services::hashset_banned_store::HashsetBannedTokenStore::new(),
        ));
        let app_state =
            auth_service::app_state::AppState::new(user_store, banned_token_store.clone());

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build application");
        let address = format!("http://{}", app.address);

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(reqwest::cookie::Jar::default());
        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .expect("Failed to build HTTP client");

        TestApp {
            address,
            cookie_jar,
            http_client,
            banned_token_store,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&self.address)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
