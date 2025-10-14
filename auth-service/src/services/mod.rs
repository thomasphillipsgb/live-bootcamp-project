pub mod hashmap_user_store;
mod data_stores;

pub use hashmap_user_store::{HashMapUserStore};
pub use data_stores::{UserStore, UserStoreError};

pub trait Storage<K, V, E> {
    fn insert(&mut self, value: V) -> Result<(), E>;
    fn get(&self, key: &K) -> Result<V, E>;
    fn validate(&self, key: &K, value: &str) -> Result<(), E>;

    async fn insert_async(&mut self, value: V) -> Result<(), E> {
        async move {
            self.insert(value)
        }.await
    }

    async fn get_async(&self, key: &K) -> Result<V, E> {
        async move {
            self.get(key)
        }.await
    }

    async fn validate_async(&self, key: &K, value: &str) -> Result<(), E> {
        async move {
            self.validate(key, value)
        }.await
    }
}