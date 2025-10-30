use std::collections::HashMap;

use crate::{
    domain::models::Email,
    services::{
        data_stores::{LoginAttemptId, TwoFACode},
        TwoFACodeStore, TwoFACodeStoreError,
    },
};

#[derive(Default)]
pub struct HashmapTwoFACodeStore {
    codes: HashMap<Email, (LoginAttemptId, TwoFACode)>,
}

impl TwoFACodeStore for HashmapTwoFACodeStore {
    async fn add_code(
        &mut self,
        email: Email,
        login_attempt_id: LoginAttemptId,
        code: TwoFACode,
    ) -> Result<(), TwoFACodeStoreError> {
        self.codes.insert(email, (login_attempt_id, code));
        Ok(())
    }

    async fn remove_code(&mut self, email: &Email) -> Result<(), TwoFACodeStoreError> {
        if self.codes.remove(email).is_some() {
            Ok(())
        } else {
            Err(TwoFACodeStoreError::LoginAttemptIdNotFound)
        }
    }

    async fn get_code(
        &self,
        email: &Email,
    ) -> Result<(LoginAttemptId, TwoFACode), TwoFACodeStoreError> {
        if let Some((login_attempt_id, code)) = self.codes.get(email) {
            Ok((login_attempt_id.clone(), code.clone()))
        } else {
            Err(TwoFACodeStoreError::LoginAttemptIdNotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        domain::models::Email,
        services::{
            data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore},
            hashmap_two_fa_code_store::HashmapTwoFACodeStore,
        },
    };

    #[tokio::test]
    async fn test_add_and_get_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let login_attempt_id =
            LoginAttemptId::new("550e8400-e29b-41d4-a716-446655440000".to_string()).unwrap();
        let code = TwoFACode::new("123456".to_string()).unwrap();

        store
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await
            .unwrap();

        let (retrieved_login_attempt_id, retrieved_code) = store.get_code(&email).await.unwrap();
        assert_eq!(retrieved_login_attempt_id, login_attempt_id);
        assert_eq!(retrieved_code, code);
    }

    #[tokio::test]
    async fn test_remove_code() {
        let mut store = HashmapTwoFACodeStore::default();
        let email = Email::new("test@example.com".to_string()).unwrap();
        let login_attempt_id =
            LoginAttemptId::new("550e8400-e29b-41d4-a716-446655440000".to_string()).unwrap();
        let code = TwoFACode::new("123456".to_string()).unwrap();

        store
            .add_code(email.clone(), login_attempt_id.clone(), code.clone())
            .await
            .unwrap();

        store.remove_code(&email).await.unwrap();
        assert!(store.get_code(&email).await.is_err());
    }
}
