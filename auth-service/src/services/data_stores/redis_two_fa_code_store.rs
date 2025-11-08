use std::sync::Arc;

use redis::{aio::MultiplexedConnection, AsyncCommands, Commands, Connection};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    domain::models::Email,
    services::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError},
};

#[derive(Clone)]
pub struct RedisTwoFACodeStore {
    connection_manager: MultiplexedConnection,
}

impl RedisTwoFACodeStore {
    pub fn new(connection_manager: MultiplexedConnection) -> Self {
        Self { connection_manager }
    }
}

impl TwoFACodeStore for RedisTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        let key = get_key(&email);
        let value = serde_json::to_string(&TwoFATuple(
            login_attempt_id.as_ref().to_string(),
            code.as_ref().to_string(),
        ))
        .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;

        let mut conn = self.connection_manager.clone();
        let _: () = conn
            .set_ex(key, value, TEN_MINUTES_IN_SECONDS)
            .await
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        let key = get_key(email);
        let mut conn = self.connection_manager.clone();
        let _: () = conn
            .del(key)
            .await
            .map_err(|_| TwoFACodeStoreError::UnexpectedError)?;

        Ok(())
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        let key = get_key(email);
        let mut conn = self.connection_manager.clone();
        let value: String = conn
            .get(key)
            .await
            .map_err(|_| TwoFACodeStoreError::LoginAttemptIdNotFound)?;
        let tuple: TwoFATuple =
            serde_json::from_str(&value).map_err(|_| TwoFACodeStoreError::UnexpectedError)?;

        Ok((
            LoginAttemptId::new(tuple.0).unwrap(),
            TwoFACode::new(tuple.1).unwrap(),
        ))
    }
}

#[derive(Serialize, Deserialize)]
struct TwoFATuple(pub String, pub String);

const TEN_MINUTES_IN_SECONDS: u64 = 600;
const TWO_FA_CODE_PREFIX: &str = "two_fa_code:";

fn get_key(email: &Email) -> String {
    format!("{}{}", TWO_FA_CODE_PREFIX, email.as_ref())
}
