pub mod hashmap_user_store;
mod data_stores;

pub use hashmap_user_store::{HashMapUserStore};
pub use data_stores::{UserStore, UserStoreError};

pub trait Storage<V, E> {
    fn insert(&mut self, key: String, value: V) -> Result<(), E>;
    fn get(&self, key: &str) -> Result<V, E>;
    fn validate(&self, key: &str, value: &str) -> Result<(), E>;

    async fn insert_async(&mut self, key: String, value: V) -> Result<(), E> {
        async move {
            self.insert(key, value)
        }.await
    }

    async fn get_async(&self, key: &str) -> Result<V, E> {
        async move {
            self.get(key)
        }.await
    }

    async fn validate_async(&self, key: &str, value: &str) -> Result<(), E> {
        async move {
            self.validate(key, value)
        }.await
    }
}