use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Metrics {
    command_usage: Arc<Mutex<HashMap<String, usize>>>,
    errors: Arc<Mutex<Vec<(DateTime<Utc>, String)>>>,
    latency: Arc<Mutex<Vec<(DateTime<Utc>, u64)>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            command_usage: Arc::new(Mutex::new(HashMap::new())),
            errors: Arc::new(Mutex::new(Vec::new())),
            latency: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn increment_command(&self, command: &str) {
        let mut usage = self.command_usage.lock().await;
        *usage.entry(command.to_string()).or_insert(0) += 1;
    }

    pub async fn log_error(&self, error: &str) {
        let mut errors = self.errors.lock().await;
        errors.push((Utc::now(), error.to_string()));
    }

    pub async fn log_latency(&self, latency: u64) {
        let mut latencies = self.latency.lock().await;
        latencies.push((Utc::now(), latency));
    }
}