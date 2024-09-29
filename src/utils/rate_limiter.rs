use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant};

pub struct RateLimiter {
    limits: HashMap<String, (u32, Duration)>,
    usage: Arc<Mutex<HashMap<(String, u64), Vec<Instant>>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            limits: HashMap::new(),
            usage: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_limit(&mut self, key: &str, count: u32, duration: Duration) {
        self.limits.insert(key.to_string(), (count, duration));
    }

    pub async fn check(&self, key: &str, user_id: u64) -> bool {
        let now = Instant::now();
        let mut usage = self.usage.lock().await;
        let user_key = (key.to_string(), user_id);

        if let Some((count, duration)) = self.limits.get(key) {
            let times = usage.entry(user_key).or_insert_with(Vec::new);
            times.retain(|&t| now.duration_since(t) < *duration);
            
            if times.len() >= *count as usize {
                false
            } else {
                times.push(now);
                true
            }
        } else {
            true
        }
    }
}