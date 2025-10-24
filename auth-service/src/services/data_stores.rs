use crate::{
    domain::{models::Email, User},
    services::Storage,
};

pub trait UserStore: Storage<Email, User, UserStoreError> {}

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
