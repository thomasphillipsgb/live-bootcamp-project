use std::collections::HashSet;

use crate::services::{data_stores::BannedTokenStore, UserStoreError};

#[derive(Clone)]
pub struct HashsetBannedTokenStore {
    banned_tokens: HashSet<String>,
}

impl HashsetBannedTokenStore {
    pub fn new() -> Self {
        Self {
            banned_tokens: HashSet::new(),
        }
    }
}
impl BannedTokenStore for HashsetBannedTokenStore {
    fn ban_token(&mut self, token: &str) -> Result<(), UserStoreError> {
        self.banned_tokens.insert(token.to_string());
        Ok(())
    }

    fn is_token_banned(&self, token: &str) -> bool {
        self.banned_tokens.contains(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ban_and_check_token() {
        let mut store = HashsetBannedTokenStore {
            banned_tokens: std::collections::HashSet::new(),
        };

        let token = "sample_token";

        // Initially, the token should not be banned
        assert_eq!(store.is_token_banned(token), false);

        // Ban the token
        store.ban_token(token).unwrap();

        // Now, the token should be banned
        assert_eq!(store.is_token_banned(token), true);
    }
}
