use crate::{domain::{models::Email, User}, services::Storage};

pub trait UserStore: Storage<Email, User, UserStoreError> {}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}
