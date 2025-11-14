pub mod email_client;
pub mod error;
pub mod mock_email_client;
pub mod user;

pub use email_client::*;
pub use error::*;
pub use user::*;

pub mod models {
    use color_eyre::eyre::eyre;
    use color_eyre::eyre::Result;
    use validator::ValidateEmail;

    #[derive(Clone, Eq, Hash, PartialEq)]
    pub struct Email(String);

    impl Email {
        pub fn new(email: String) -> Result<Self> {
            if email.validate_email() {
                Ok(Self(email))
            } else {
                Err(eyre!("Invalid email format"))
            }
        }
    }

    impl AsRef<str> for Email {
        fn as_ref(&self) -> &str {
            &self.0
        }
    }

    #[derive(Clone)]
    pub struct Password(String);

    impl Password {
        pub fn new(password: String) -> Result<Self> {
            if password.trim().is_empty() || password.len() < 8 {
                return Err(eyre!("Invalid password"));
            }
            // Add password validation logic if needed
            Ok(Self(password))
        }
    }

    impl AsRef<str> for Password {
        fn as_ref(&self) -> &str {
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
