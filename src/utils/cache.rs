use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::hash::Hash;

pub struct CacheEntry<T> {
    data: T,
    expires_at: Instant,
}

pub struct Cache<K, V> {
    data: Arc<RwLock<HashMap<K, CacheEntry<V>>>>,
    ttl: Duration,
}

impl<K, V> Cache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn new(ttl: Duration) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        let data = self.data.read().await;
        if let Some(entry) = data.get(key) {
            if entry.expires_at > Instant::now() {
                return Some(entry.data.clone());
            }
        }
        None
    }

    pub async fn set(&self, key: K, value: V) {
        let mut data = self.data.write().await;
        data.insert(key, CacheEntry {
            data: value,
            expires_at: Instant::now() + self.ttl,
        });
    }

    pub async fn remove(&self, key: &K) {
        let mut data = self.data.write().await;
        data.remove(key);
    }

    pub async fn clear(&self) {
        let mut data = self.data.write().await;
        data.clear();
    }

    pub async fn cleanup(&self) {
        let mut data = self.data.write().await;
        data.retain(|_, v| v.expires_at > Instant::now());
    }

    pub async fn get_or_insert_with<F>(&self, key: K, f: F) -> V
    where
        F: FnOnce() -> V,
    {
        if let Some(value) = self.get(&key).await {
            return value;
        }

        let value = f();
        self.set(key, value.clone()).await;
        value
    }
}