pub mod domain;
pub mod routes;
pub mod services;
pub mod utils;

use std::error::Error;

use axum::{
    http::{self, Method},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    serve::Serve,
    Json, Router,
};
use redis::{Client, RedisResult};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use sqlx::{database, postgres::PgPoolOptions};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing::info;

use crate::{
    app_state::AppState,
    domain::{AuthAPIError, EmailClient},
    routes::{
        login_handler, logout_handler, signup_handler, verify_2fa_handler, verify_token_handler,
    },
    services::{BannedTokenStore, TwoFACodeStore, UserStore},
    utils::tracing::{make_span_with_request_id, on_request, on_response},
};

pub struct Application {
    server: Serve<tokio::net::TcpListener, Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build<T, U, V, W>(
        app_state: AppState<T, U, V, W>,
        address: &str,
    ) -> Result<Self, Box<dyn Error>>
    where
        T: UserStore + Clone + Send + Sync + 'static,
        U: BannedTokenStore + Clone + Send + Sync + 'static,
        V: TwoFACodeStore + Clone + Send + Sync + 'static,
        W: EmailClient + Clone + Send + Sync + 'static,
    {
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            // TODO: Replace [YOUR_DROPLET_IP] with your Droplet IP address
            "http://161.35.46.112:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            // Allow GET and POST requests
            .allow_methods([Method::GET, Method::POST])
            // Allow cookies to be included in requests
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .fallback_service(ServeDir::new("assets"))
            .route("/signup", post(signup_handler))
            .route("/login", post(login_handler))
            .route("/logout", post(logout_handler))
            .route("/verify-2fa", post(verify_2fa_handler))
            .route("/verify-token", post(verify_token_handler))
            .with_state(app_state)
            .layer(cors)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(make_span_with_request_id)
                    .on_request(on_request)
                    .on_response(on_response),
            );

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server: Serve<tokio::net::TcpListener, Router, Router> = axum::serve(listener, router);

        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        info!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        log_error_chain(&self);
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (http::StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => {
                (http::StatusCode::BAD_REQUEST, "Invalid credentials")
            }
            AuthAPIError::UnexpectedError(_) => {
                (http::StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
            AuthAPIError::IncorrectCredentials => {
                (http::StatusCode::UNAUTHORIZED, "Incorrect credentials")
            }
            AuthAPIError::MissingToken => (http::StatusCode::BAD_REQUEST, "Missing token"),
            AuthAPIError::InvalidToken => (http::StatusCode::UNAUTHORIZED, "Invalid token"),
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });
        (status, body).into_response()
    }
}

fn log_error_chain(e: &(dyn Error + 'static)) {
    let separator =
        "\n-----------------------------------------------------------------------------------\n";
    let mut report = format!("{}{:?}\n", separator, e);
    let mut current = e.source();
    while let Some(cause) = current {
        let str = format!("Caused by:\n\n{:?}", cause);
        report = format!("{}\n{}", report, str);
        current = cause.source();
    }
    report = format!("{}\n{}", report, separator);
    tracing::error!("{}", report);
}

pub async fn get_postgres_pool(database_url: &SecretString) -> Result<sqlx::PgPool, sqlx::Error> {
    let database_url = database_url.expose_secret();
    println!("Connecting to Postgres at {}", database_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    Ok(pool)
}

pub fn get_redis_client(redis_hostname: String) -> RedisResult<Client> {
    let redis_url = format!("redis://{}", redis_hostname);
    Client::open(redis_url)
}

pub mod app_state {
    use std::sync::Arc;
    use tokio::sync::RwLock;

    use crate::domain::EmailClient;
    use crate::services::BannedTokenStore;
    use crate::services::TwoFACodeStore;
    use crate::services::UserStore;

    // Using a type alias to improve readability!
    pub type UserStoreType<T> = Arc<RwLock<T>>;
    pub type BannedTokenStoreType<U> = Arc<RwLock<U>>;
    pub type TwoFACodeStoreType<V> = Arc<RwLock<V>>;
    pub type EmailClientType<W> = Arc<RwLock<W>>;

    #[derive(Clone)]
    pub struct AppState<T, U, V, W>
    where
        T: UserStore,
        U: BannedTokenStore,
        V: TwoFACodeStore,
        W: EmailClient,
    {
        pub user_store: UserStoreType<T>,
        pub banned_token_store: BannedTokenStoreType<U>,
        pub two_fa_code_store: TwoFACodeStoreType<V>,
        pub email_client: EmailClientType<W>,
    }

    impl<T, U, V, W> AppState<T, U, V, W>
    where
        T: UserStore,
        U: BannedTokenStore,
        V: TwoFACodeStore,
        W: EmailClient,
    {
        pub fn new(
            user_store: UserStoreType<T>,
            banned_token_store: BannedTokenStoreType<U>,
            two_fa_code_store: TwoFACodeStoreType<V>,
            email_client: EmailClientType<W>,
        ) -> Self {
            Self {
                user_store,
                banned_token_store,
                two_fa_code_store,
                email_client,
            }
        }
    }
}
