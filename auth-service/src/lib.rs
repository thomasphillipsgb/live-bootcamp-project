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
use serde::{Deserialize, Serialize};
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::{
    app_state::AppState,
    domain::{models::Email, AuthAPIError},
    routes::{
        login_handler, logout_handler, signup_handler, verify_2fa_handler, verify_token_handler,
    },
    services::{BannedTokenStore, UserStore},
};

pub struct Application {
    server: Serve<tokio::net::TcpListener, Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build<T, U>(
        app_state: AppState<T, U>,
        address: &str,
    ) -> Result<Self, Box<dyn Error>>
    where
        T: UserStore + Clone + Send + Sync + 'static,
        U: BannedTokenStore + Clone + Send + Sync + 'static,
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
            .route("/hello", get(hello_handler))
            .route("/signup", post(signup_handler))
            .route("/login", post(login_handler))
            .route("/logout", post(logout_handler))
            .route("/verify-2fa", post(verify_2fa_handler))
            .route("/verify-token", post(verify_token_handler))
            .with_state(app_state)
            .layer(cors);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server: Serve<tokio::net::TcpListener, Router, Router> = axum::serve(listener, router);

        Ok(Self { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

async fn hello_handler() -> Html<&'static str> {
    println!("hello handler called");
    Html("<h1>Hello, Rustaceans!</h1>")
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (http::StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => {
                (http::StatusCode::BAD_REQUEST, "Invalid credentials")
            }
            AuthAPIError::UnexpectedError => {
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

pub mod app_state {
    use std::sync::Arc;
    use tokio::sync::RwLock;

    use crate::domain::models::Email;
    use crate::services::BannedTokenStore;
    use crate::services::UserStore;

    // Using a type alias to improve readability!
    pub type UserStoreType<T> = Arc<RwLock<T>>;
    pub type BannedTokenStoreType<U> = Arc<RwLock<U>>;

    #[derive(Clone)]
    pub struct AppState<T, U>
    where
        T: UserStore,
        U: BannedTokenStore,
    {
        pub user_store: UserStoreType<T>,
        pub banned_token_store: BannedTokenStoreType<U>,
    }

    impl<T, U> AppState<T, U>
    where
        T: UserStore,
        U: BannedTokenStore,
    {
        pub fn new(
            user_store: UserStoreType<T>,
            banned_token_store: BannedTokenStoreType<U>,
        ) -> Self {
            Self {
                user_store,
                banned_token_store,
            }
        }
    }
}
