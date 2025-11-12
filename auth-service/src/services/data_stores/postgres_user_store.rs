use std::{error::Error, future::Future};

use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};

use sqlx::PgPool;

use crate::{
    domain::{
        models::{Email, Password},
        User,
    },
    services::{UserStore, UserStoreError},
};

#[derive(Clone)]
pub struct PostgresUserStore {
    pool: PgPool,
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserStore for PostgresUserStore {
    #[tracing::instrument(name = "Adding user to PostgreSQL", skip_all)]
    async fn insert(&mut self, value: crate::domain::User) -> Result<(), super::UserStoreError> {
        let mut connection = self
            .pool
            .acquire()
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        let executor = &mut *connection;

        let password_hash = compute_password_hash(value.password.as_ref().to_string())
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        let result = sqlx::query!(
            r#"
            INSERT INTO users (email, password_hash, requires_2fa)
            VALUES ($1, $2, $3)
            "#,
            value.email.as_ref(),
            password_hash,
            value.requires_2fa
        )
        .execute(executor)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(db_err)) if db_err.code() == Some("23505".into()) => {
                Err(UserStoreError::UserAlreadyExists)
            }
            Err(_) => Err(UserStoreError::UnexpectedError),
        }
    }

    #[tracing::instrument(name = "Retrieving user from PostgreSQL", skip_all)]
    async fn get(&self, key: &crate::domain::models::Email) -> Result<User, super::UserStoreError> {
        let mut connection = self
            .pool
            .acquire()
            .await
            .map_err(|_| UserStoreError::UnexpectedError)?;

        let executor = &mut *connection;

        sqlx::query!(
            r#"
            SELECT email, password_hash, requires_2fa
            FROM users
            WHERE email = $1
            "#,
            key.as_ref()
        )
        .fetch_one(executor)
        .await
        .map_err(|_| UserStoreError::UserNotFound)
        .map(|record| {
            User::new(
                Email::new(record.email).unwrap(),
                Password::new(record.password_hash).unwrap(),
                record.requires_2fa,
            )
        })
    }

    #[tracing::instrument(name = "Validating user credentials in PostgreSQL", skip_all)]
    async fn validate(
        &self,
        key: &crate::domain::models::Email,
        value: &str,
    ) -> Result<(), super::UserStoreError> {
        let user = self.get(key).await?;

        verify_password_hash(user.password.as_ref().to_string(), value.to_string())
            .await
            .map_err(|_| UserStoreError::InvalidCredentials)
    }
    // TODO: Implement all required methods. Note that you will need to make SQL queries against our PostgreSQL instance inside these methods.
}

#[tracing::instrument(name = "Verify password hash", skip_all)]
async fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), Box<dyn Error>> {
    tokio::task::spawn_blocking(move || {
        let expected_password_hash: PasswordHash<'_> = PasswordHash::new(&expected_password_hash)?;

        Argon2::default().verify_password(password_candidate.as_bytes(), &expected_password_hash)
    })
    .await??;

    Ok(())
}

#[tracing::instrument(name = "Computing password hash", skip_all)]
async fn compute_password_hash(password: String) -> Result<String, Box<dyn Error>> {
    let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
    let hasher = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None)?,
    );
    let password_hash: Result<String, argon2::password_hash::Error> =
        tokio::task::spawn_blocking(move || {
            Ok(hasher
                .hash_password(password.as_bytes(), &salt)?
                .to_string())
        })
        .await?;

    Ok(password_hash?)
}
