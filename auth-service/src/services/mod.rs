pub mod hashmap_user_store;

pub use hashmap_user_store::{HashMapUserStore, UserStoreError};

pub trait Storage<V, E> {
    fn insert(&mut self, key: String, value: V) -> Result<(), E>;
    fn get(&self, key: &str) -> Result<V, E>;
}