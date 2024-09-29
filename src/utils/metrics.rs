use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Metrics {
    command_usage: Arc<Mutex<HashMap<String, usize>>>,
    errors: Arc<Mutex<Vec<(DateTime<Utc>, String)>>>,
    latency: Arc<Mutex<Vec<(DateTime<Utc>, u64)>>>,
    events: Arc<Mutex<HashMap<String, usize>>>,
    gauges: Arc<Mutex<HashMap<String, f64>>>,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            command_usage: Arc::new(Mutex::new(HashMap::new())),
            errors: Arc::new(Mutex::new(Vec::new())),
            latency: Arc::new(Mutex::new(Vec::new())),
            events: Arc::new(Mutex::new(HashMap::new())),
            gauges: Arc::new(Mutex::new(HashMap::new())),
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

    pub async fn log_event(&self, event: &str) {
        let mut events = self.events.lock().await;
        *events.entry(event.to_string()).or_insert(0) += 1;
    }

    pub async fn set_gauge(&self, name: &str, value: f64) {
        let mut gauges = self.gauges.lock().await;
        gauges.insert(name.to_string(), value);
    }

    pub async fn get_command_usage(&self) -> HashMap<String, usize> {
        self.command_usage.lock().await.clone()
    }

    pub async fn get_error_count(&self) -> usize {
        self.errors.lock().await.len()
    }

    pub async fn get_average_latency(&self) -> Option<f64> {
        let latencies = self.latency.lock().await;
        if latencies.is_empty() {
            None
        } else {
            Some(latencies.iter().map(|&(_, l)| l as f64).sum::<f64>() / latencies.len() as f64)
        }
    }

    pub async fn get_event_counts(&self) -> HashMap<String, usize> {
        self.events.lock().await.clone()
    }

    pub async fn get_gauges(&self) -> HashMap<String, f64> {
        self.gauges.lock().await.clone()
    }
}