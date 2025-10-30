use std::future::Future;

use crate::domain::{models::Email, User};

// Email, crate::domain::User, crate::services::UserStoreError

pub trait UserStore {
    fn insert(&mut self, value: User) -> impl Future<Output = Result<(), UserStoreError>> + Send;
    fn get(&self, key: &Email) -> impl Future<Output = Result<User, UserStoreError>> + Send;
    fn validate(
        &self,
        key: &Email,
        value: &str,
    ) -> impl Future<Output = Result<(), UserStoreError>> + Send;
}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

pub trait BannedTokenStore {
    fn ban_token(&mut self, token: &str) -> Result<(), UserStoreError>;
    fn is_token_banned(&self, token: &str) -> bool;
}
