use redis::AsyncCommands;

use crate::{
    services::{data_stores::BannedTokenStoreError, BannedTokenStore},
    utils::auth::TOKEN_TTL_SECONDS,
};

#[derive(Clone)]
pub struct RedisBannedTokenStore {
    connection_manager: redis::aio::MultiplexedConnection,
}

impl RedisBannedTokenStore {
    pub fn new(connection_manager: redis::aio::MultiplexedConnection) -> Self {
        Self { connection_manager }
    }
}

impl BannedTokenStore for RedisBannedTokenStore {
    async fn ban_token(&mut self, token: &str) -> Result<(), BannedTokenStoreError> {
        let key = get_key(token);

        let mut conn = self.connection_manager.clone();
        let _: () = conn
            .set_ex(key, true, TOKEN_TTL_SECONDS)
            .await
            .map_err(|_| BannedTokenStoreError::UnexpectedError)?;
        Ok(())
    }

    async fn is_token_banned(&self, token: &str) -> bool {
        let key = get_key(token);

        let mut conn = self.connection_manager.clone();
        let result = conn.exists(key).await;
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
