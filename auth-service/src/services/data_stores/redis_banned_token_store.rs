use std::sync::Arc;

use redis::{Commands, Connection};
use tokio::sync::RwLock;

use crate::{
    services::{data_stores::BannedTokenStoreError, BannedTokenStore},
    utils::auth::TOKEN_TTL_SECONDS,
};

#[derive(Clone)]
pub struct RedisBannedTokenStore {
    conn: Arc<RwLock<Connection>>,
}

impl RedisBannedTokenStore {
    pub fn new(conn: Arc<RwLock<Connection>>) -> Self {
        Self { conn }
    }
}

impl BannedTokenStore for RedisBannedTokenStore {
    async fn ban_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError> {
        let key = get_key(token);

        let mut conn = self.conn.write().await;
        let _: () = conn
            .set_ex(key, true, TOKEN_TTL_SECONDS)
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;
        Ok(())
    }

    async fn is_token_banned(&self, token: &str) -> bool {
        let key = get_key(token);

        let mut connection = self.conn.write().await;
        let result: Result<bool, _> = connection.exists(key);
        match result {
            Ok(exists) => exists,
            Err(_) => false,
        }
    }
}

// We are using a key prefix to prevent collisions and organize data!
const BANNED_TOKEN_KEY_PREFIX: &str = "banned_token:";

fn get_key(token: &str) -> String {
    format!("{}{}", BANNED_TOKEN_KEY_PREFIX, token)
}
