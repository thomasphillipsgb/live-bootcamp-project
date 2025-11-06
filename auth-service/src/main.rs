use std::sync::Arc;

use auth_service::{
    domain::mock_email_client::MockEmailClient,
    get_postgres_pool,
    services::data_stores::{
        hashset_banned_store::HashsetBannedTokenStore, HashMapUserStore, HashmapTwoFACodeStore,
    },
    utils::constants::{prod, DATABASE_URL},
    Application,
};
use sqlx::PgPool;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(HashMapUserStore::new()));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::new()));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::new()));
    let email_client = Arc::new(RwLock::new(MockEmailClient {}));

    let pg_pool = configure_postgresql().await;

    let app_state = auth_service::app_state::AppState::new(
        user_store.clone(),
        banned_token_store.clone(),
        two_fa_code_store.clone(),
        email_client.clone(),
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build application");

    app.run().await.expect("Failed to run application");
}

async fn configure_postgresql() -> PgPool {
    // Create a new database connection pool
    let pg_pool = get_postgres_pool(&DATABASE_URL)
        .await
        .expect("Failed to create Postgres connection pool!");

    // Run database migrations against our test database!
    sqlx::migrate!()
        .run(&pg_pool)
        .await
        .expect("Failed to run migrations");

    pg_pool
}
