use dotenvy::dotenv;
use lazy_static::lazy_static;
use secrecy::SecretString;
use std::env as std_env;

pub const JWT_COOKIE_NAME: &str = "jwt";
pub const DEFAULT_REDIS_HOSTNAME: &str = "127.0.0.1";

// Define a lazily evaluated static. lazy_static is needed because std_env::var is not a const function.
lazy_static! {
    pub static ref JWT_SECRET: SecretString = set_token();
    pub static ref RESEND_SECRET: SecretString = set_resend_secret();
    pub static ref DATABASE_URL: SecretString = set_url();
    pub static ref REDIS_HOST_NAME: String = set_redis_host();
    pub static ref SENDER_EMAIL: SecretString = set_sender_email();
}

fn set_sender_email() -> SecretString {
    dotenv().ok(); // Load environment variables
    let email = std_env::var(env::SENDER_EMAIL_ENV_VAR).expect("SENDER_EMAIL must be set.");
    if email.is_empty() {
        panic!("SENDER_EMAIL must not be empty.");
    }
    email.into()
}

fn set_resend_secret() -> SecretString {
    dotenv().ok(); // Load environment variables
    let secret = std_env::var(env::RESEND_SECRET_ENV_VAR).expect("RESEND_API_KEY must be set.");
    if secret.is_empty() {
        panic!("RESEND_API_KEY must not be empty.");
    }
    secret.into()
}

fn set_url() -> SecretString {
    dotenv().ok(); // Load environment variables
    let url = std_env::var(env::DATABASE_URL_ENV_VAR).expect("DATABASE_URL must be set.");
    if url.is_empty() {
        panic!("DATABASE_URL must not be empty.");
    }
    url.into()
}

fn set_token() -> SecretString {
    dotenv().ok(); // Load environment variables
    let secret = std_env::var(env::JWT_SECRET_ENV_VAR).expect("JWT_SECRET must be set.");
    if secret.is_empty() {
        panic!("JWT_SECRET must not be empty.");
    }
    secret.into()
}

fn set_redis_host() -> String {
    dotenv().ok();
    std_env::var(env::REDIS_HOST_NAME_ENV_VAR).unwrap_or(DEFAULT_REDIS_HOSTNAME.to_owned())
}

pub mod env {
    pub const JWT_SECRET_ENV_VAR: &str = "JWT_SECRET";
    pub const RESEND_SECRET_ENV_VAR: &str = "RESEND_API_KEY";
    pub const DATABASE_URL_ENV_VAR: &str = "DATABASE_URL";
    pub const REDIS_HOST_NAME_ENV_VAR: &str = "REDIS_HOST_NAME";
    pub const SENDER_EMAIL_ENV_VAR: &str = "SENDER_EMAIL";
}

pub mod prod {
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
}

pub mod test {
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
}
