use std::future::Future;

use rand::{Rng, RngCore};

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

pub trait TwoFACodeStore {
    fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> impl Future<Output = Result<(), TwoFACodeStoreError>> + Send;
    fn remove_code(
        &mut self,
        email: &Email,
    ) -> impl Future<Output = Result<(), TwoFACodeStoreError>> + Send;
    fn get_code(
        &self,
        email: &Email,
    ) -> impl Future<Output = Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError>> + Send;
}

#[derive(Debug, PartialEq)]
pub enum TwoFACodeStoreError {
    LoginAttemptIdNotFound,
    UnexpectedError,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttemptId(String);

impl LoginAttemptId {
    pub fn new(id: String) -> Result<Self, String> {
        if let Ok(_) = uuid::Uuid::parse_str(&id) {
            Ok(LoginAttemptId(id))
        } else {
            Err("Invalid UUID format".to_string())
        }
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        LoginAttemptId(uuid::Uuid::new_v4().to_string())
    }
}

impl AsRef<str> for LoginAttemptId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn new(code: String) -> Result<Self, String> {
        // Ensure `code` is a valid 6-digit code
        if code.len() == 6 && code.chars().all(char::is_numeric) {
            Ok(TwoFACode(code))
        } else {
            Err("Invalid 2FA code".to_string())
        }
    }
}

impl Default for TwoFACode {
    fn default() -> Self {
        // Use the `rand` crate to generate a random 2FA code.
        // The code should be 6 digits (ex: 834629)
        let code = rand::rng().random_range(100000..999999).to_string();
        TwoFACode(code)
    }
}

impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{LoginAttemptId, TwoFACode};

    #[test]
    fn test_login_attempt_id() {
        let valid_id = LoginAttemptId::new("550e8400-e29b-41d4-a716-446655440000".to_string());
        assert!(valid_id.is_ok());

        let invalid_id = LoginAttemptId::new("invalid-uuid".to_string());
        assert!(invalid_id.is_err());
    }

    #[test]
    fn test_two_fa_code() {
        let valid_code = TwoFACode::new("123456".to_string());
        assert!(valid_code.is_ok());

        let invalid_code = TwoFACode::new("invalid".to_string());
        assert!(invalid_code.is_err());
    }
}
