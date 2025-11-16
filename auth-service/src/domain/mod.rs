pub mod email_client;
pub mod error;
pub mod mock_email_client;
pub mod resend_email_client;
pub mod user;

pub use email_client::*;
pub use error::*;
pub use user::*;

pub mod models {
    use color_eyre::eyre::eyre;
    use color_eyre::eyre::Result;
    use secrecy::ExposeSecret;
    use secrecy::SecretString;
    use std::hash::Hash;
    use validator::ValidateEmail;

    #[derive(Clone)]
    pub struct Email(SecretString);

    impl Email {
        pub fn new(email: SecretString) -> Result<Self> {
            if email.expose_secret().validate_email() {
                Ok(Self(email))
            } else {
                Err(eyre!("Invalid email format"))
            }
        }
    }

    impl PartialEq for Email {
        fn eq(&self, other: &Self) -> bool {
            self.0.expose_secret() == other.0.expose_secret()
        }
    }

    impl Hash for Email {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.0.expose_secret().hash(state);
        }
    }

    impl Eq for Email {}

    impl AsRef<SecretString> for Email {
        fn as_ref(&self) -> &SecretString {
            &self.0
        }
    }

    #[derive(Clone)]
    pub struct Password(SecretString);

    impl Password {
        pub fn new(password: SecretString) -> Result<Self> {
            let password = password.expose_secret();
            if password.trim().is_empty() || password.len() < 8 {
                return Err(eyre!("Invalid password"));
            }
            // Add password validation logic if needed
            Ok(Self(SecretString::new(password.into())))
        }
    }

    impl PartialEq for Password {
        fn eq(&self, other: &Self) -> bool {
            self.0.expose_secret() == other.0.expose_secret()
        }
    }

    impl AsRef<SecretString> for Password {
        fn as_ref(&self) -> &SecretString {
            &self.0
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn test_email_validation() {
            assert!(super::Email::new("test@example.com".into()).is_ok());
            assert!(super::Email::new("invalid-email".into()).is_err());
        }
    }
}
