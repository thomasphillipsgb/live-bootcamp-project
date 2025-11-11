use std::sync::Arc;

use auth_service::{
    Application, domain::mock_email_client::MockEmailClient, get_postgres_pool, get_redis_client, services::data_stores::{
        HashmapTwoFACodeStore, postgres_user_store::PostgresUserStore, redis_banned_token_store::RedisBannedTokenStore, redis_two_fa_code_store::RedisTwoFACodeStore
    }, utils::{constants::{DATABASE_URL, REDIS_HOST_NAME, prod}, tracing::init_tracing}
};
use sqlx::PgPool;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    init_tracing();
    let pg_pool = configure_postgresql().await;
    let redis_connection = configure_redis().await;

    let banned_token_store = Arc::new(RwLock::new(RedisBannedTokenStore::new(
        redis_connection.clone(),
    )));
    let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
    let two_fa_code_store = Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis_connection)));
    let email_client = Arc::new(RwLock::new(MockEmailClient {}));

    let app_state = auth_service::app_state::AppState::new(
        user_store,
        banned_token_store,
        two_fa_code_store,
        email_client,
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

async fn configure_redis() -> redis::aio::MultiplexedConnection {
    get_redis_client(REDIS_HOST_NAME.to_owned())
        .expect("Failed to get Redis client")
        .get_multiplexed_async_connection()
        .await
        .expect("Failed to get Redis connection")
}
