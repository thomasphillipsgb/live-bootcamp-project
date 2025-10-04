pub mod routes;

use std::error::Error;

use axum::{
    response::Html,
    routing::{get, post},
    serve::Serve,
    Router,
};
use tower_http::services::ServeDir;

use crate::routes::{
    login_handler, logout_handler, signup_handler, verify_2fa_handler, verify_token_handler,
};

pub struct Application {
    server: Serve<tokio::net::TcpListener, Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            .fallback_service(ServeDir::new("assets"))
            .route("/hello", get(hello_handler))
            .route("/signup", post(signup_handler))
            .route("/login", post(login_handler))
            .route("/logout", post(logout_handler))
            .route("/verify-2fa", post(verify_2fa_handler))
            .route("/verify-token", post(verify_token_handler));

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
