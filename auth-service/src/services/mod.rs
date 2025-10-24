mod data_stores;
pub mod hashmap_user_store;
pub mod hashset_banned_store;

pub use data_stores::{BannedTokenStore, UserStore, UserStoreError};
pub use hashmap_user_store::HashMapUserStore;

pub trait Storage<K, V, E> {
    fn insert(&mut self, value: V) -> Result<(), E>;
    fn get(&self, key: &K) -> Result<V, E>;
    fn validate(&self, key: &K, value: &str) -> Result<(), E>;

    fn insert_async(&mut self, value: V) -> impl std::future::Future<Output = Result<(), E>> {
        async move { self.insert(value) }
    }

    fn get_async(&self, key: &K) -> impl std::future::Future<Output = Result<V, E>> {
        async move { self.get(key) }
    }

    fn validate_async(
        &self,
        key: &K,
        value: &str,
    ) -> impl std::future::Future<Output = Result<(), E>> {
        async move { self.validate(key, value) }
    }
}
