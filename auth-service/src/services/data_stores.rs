use crate::{domain::User, services::Storage};

pub trait UserStore: Storage<User, UserStoreError> {}

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}
