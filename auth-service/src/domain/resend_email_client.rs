use color_eyre::eyre::Result;
use resend_rs::{types::CreateEmailBaseOptions, Resend};
use secrecy::{ExposeSecret, SecretString};

use crate::domain::{models::Email, EmailClient};

#[derive(Clone)]
pub struct ResendEmailClient {
    sender: Email, // Sender's email address
    resend: Resend,
}

impl ResendEmailClient {
    pub fn new(sender: Email, authorization_token: &SecretString) -> Self {
        Self {
            sender,
            resend: Resend::new(authorization_token.expose_secret()),
        }
    }
}

impl EmailClient for ResendEmailClient {
    #[tracing::instrument(name = "Sending email", skip_all)]
    async fn send_email(&self, recipient: &Email, subject: &str, content: &str) -> Result<()> {
        let resend = &self.resend;

        let email = CreateEmailBaseOptions::new(
            self.sender.as_ref().expose_secret(),
            vec![recipient.as_ref().expose_secret()],
            subject,
        )
        .with_text(content);

        resend.emails.send(email).await?;

        Ok(())
    }
}
