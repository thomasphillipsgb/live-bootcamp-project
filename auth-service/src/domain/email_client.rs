use std::future::Future;

use super::models::Email;

// This trait represents the interface all concrete email clients should implement
pub trait EmailClient {
    fn send_email(
        &self,
        recipient: &Email,
        subject: &str,
        content: &str,
    ) -> impl Future<Output = Result<(), String>> + Send;
}
