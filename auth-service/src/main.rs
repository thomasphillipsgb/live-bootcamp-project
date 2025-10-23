use std::sync::Arc;

use auth_service::{utils::constants::prod, Application};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(
        auth_service::services::hashmap_user_store::HashMapUserStore::new(),
    ));
    let app_state = auth_service::app_state::AppState::new(user_store);
    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build application");

    app.run().await.expect("Failed to run application");
}
