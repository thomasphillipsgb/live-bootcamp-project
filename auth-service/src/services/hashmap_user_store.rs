use std::{collections::HashMap};

use crate::{domain::User, services::{Storage, UserStore, UserStoreError}};

#[derive(Clone)]
pub struct HashMapUserStore {
    users: HashMap<String, User>,
}

impl UserStore for HashMapUserStore {}

impl Storage<User, UserStoreError> for HashMapUserStore {
    fn insert(&mut self, key: String, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(key, user);
        Ok(())
    }

    fn get(&self, key: &str) -> Result<User, UserStoreError> {
        if self.users.contains_key(key) {
            Ok(self.users.get(key).unwrap().clone())
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }

    fn validate(&self, key: &str, password: &str) -> Result<(), UserStoreError> {
        if let Some(user) = self.users.get(key) {
            if user.password == password {
                Ok(())
            } else {
                Err(UserStoreError::InvalidCredentials)
            }
        } else {
            Err(UserStoreError::UserNotFound)
        }
    }
}

impl HashMapUserStore {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        self.insert(user.email.clone(), user)
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.get(email)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_user() {
        let mut store = HashMapUserStore::new();
        let user = User::new("test@example.com".into(), "password".into(), false);
        assert!(store.add_user(user).is_ok());
    }

    #[test]
    fn test_get_user() {
        let mut store = HashMapUserStore::new();
        let user = User::new("test@example.com".into(), "password".into(), false);
        store.add_user(user).unwrap();
        assert!(store.get_user("test@example.com").is_ok());
    }

    #[test]
    fn test_validate_user() {
        let mut store = HashMapUserStore::new();
        let user = User::new("test@example.com".into(), "password".into(), false);
        store.add_user(user).unwrap();
        assert!(store.validate("test@example.com", "password").is_ok());
        assert!(store.validate("test@example.com", "wrong_password").is_err());
    }
}