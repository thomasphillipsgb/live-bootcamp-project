use std::sync::Arc;

use auth_service::{
    services::hashset_banned_store::HashsetBannedTokenStore, utils::constants::prod, Application,
};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = Arc::new(RwLock::new(
        auth_service::services::hashmap_user_store::HashMapUserStore::new(),
    ));
    let banned_token_store = Arc::new(RwLock::new(HashsetBannedTokenStore::new()));
    let two_fa_code_store = Arc::new(RwLock::new(
        auth_service::services::HashmapTwoFACodeStore::new(),
    ));

    let app_state = auth_service::app_state::AppState::new(
        user_store.clone(),
        banned_token_store.clone(),
        two_fa_code_store.clone(),
    );

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build application");

    app.run().await.expect("Failed to run application");
}
