use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

pub struct RateLimiter {
    limits: Arc<Mutex<HashMap<String, (u32, Duration)>>>,
    usage: Arc<Mutex<HashMap<(String, u64), Vec<Instant>>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            limits: Arc::new(Mutex::new(HashMap::new())),
            usage: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_limit(&self, key: &str, count: u32, duration: Duration) {
        let mut limits = self.limits.lock().await;
        limits.insert(key.to_string(), (count, duration));
    }

    pub async fn check(&self, key: &str, user_id: u64) -> bool {
        let now = Instant::now();
        let mut usage = self.usage.lock().await;
        let limits = self.limits.lock().await;

        let user_key = (key.to_string(), user_id);

        if let Some(&(count, duration)) = limits.get(key) {
            let times = usage.entry(user_key).or_insert_with(Vec::new);
            times.retain(|&t| now.duration_since(t) < duration);
            
            if times.len() >= count as usize {
                false
            } else {
                times.push(now);
                true
            }
        } else {
            true
        }
    }

    pub async fn get_remaining(&self, key: &str, user_id: u64) -> Option<u32> {
        let now = Instant::now();
        let usage = self.usage.lock().await;
        let limits = self.limits.lock().await;

        let user_key = (key.to_string(), user_id);

        if let Some(&(count, duration)) = limits.get(key) {
            if let Some(times) = usage.get(&user_key) {
                let valid_times: Vec<_> = times.iter().filter(|&&t| now.duration_since(t) < duration).collect();
                Some(count.saturating_sub(valid_times.len() as u32))
            } else {
                Some(count)
            }
        } else {
            None
        }
    }

    pub async fn reset(&self, key: &str, user_id: u64) {
        let mut usage = self.usage.lock().await;
        usage.remove(&(key.to_string(), user_id));
    }

    pub async fn clean_up(&self) {
        let now = Instant::now();
        let mut usage = self.usage.lock().await;
        let limits = self.limits.lock().await;

        usage.retain(|(key, _), times| {
            if let Some(&(_, duration)) = limits.get(key) {
                times.retain(|&t| now.duration_since(t) < duration);
                !times.is_empty()
            } else {
                false
            }
        });
    }
}