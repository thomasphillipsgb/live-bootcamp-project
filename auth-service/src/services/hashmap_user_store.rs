use std::collections::HashMap;

use crate::{
    domain::{models::Email, User},
    services::{Storage, UserStore, UserStoreError},
};

#[derive(Clone)]
pub struct HashMapUserStore {
    users: HashMap<Email, User>,
}

impl UserStore for HashMapUserStore {}

impl Storage<Email, User, UserStoreError> for HashMapUserStore {
    fn insert(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    fn get(&self, key: &Email) -> Result<User, UserStoreError> {
        if self.users.contains_key(key) {
            Ok(self.users.get(key).unwrap().clone())
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    fn validate(&self, key: &Email, password: &str) -> Result<(), UserStoreError> {
        if let Some(user) = self.users.get(key) {
            if user.password.as_ref() == password {
                Ok(())
            } else {
                Err(UserStoreError::InvalidCredentials)
            }
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }
}

impl Default for HashMapUserStore {
    fn default() -> Self {
        Self::new()
    }
}

impl HashMapUserStore {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::models::Password;

    use super::*;

    #[test]
    fn test_add_user() {
        let mut store = HashMapUserStore::new();
        let user = User::new(
            Email::new("test@example.com".into()).unwrap(),
            Password::new("password".into()).unwrap(),
            false,
        );
        assert!(store.insert(user).is_ok());
    }

    #[test]
    fn test_get_user() {
        let mut store = HashMapUserStore::new();
        let user = User::new(
            Email::new("test@example.com".into()).unwrap(),
            Password::new("password".into()).unwrap(),
            false,
        );
        store.insert(user).unwrap();
        assert!(store
            .get(&Email::new("test@example.com".into()).unwrap())
            .is_ok());
    }

    #[test]
    fn test_validate_user() {
        let mut store = HashMapUserStore::new();
        let user = User::new(
            Email::new("test@example.com".into()).unwrap(),
            Password::new("password".into()).unwrap(),
            false,
        );
        store.insert(user).unwrap();
        assert!(store
            .validate(&Email::new("test@example.com".into()).unwrap(), "password")
            .is_ok());
        assert!(store
            .validate(
                &Email::new("test@example.com".into()).unwrap(),
                "wrong_password"
            )
            .is_err());
    }
}
