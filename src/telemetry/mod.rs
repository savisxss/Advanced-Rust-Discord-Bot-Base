use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};
use serde_json::json;
use crate::bot::error::BotResult;

pub struct TelemetryManager {
    events: Arc<Mutex<Vec<TelemetryEvent>>>,
    config: TelemetryConfig,
}

struct TelemetryEvent {
    timestamp: DateTime<Utc>,
    event_type: String,
    data: serde_json::Value,
}

pub struct TelemetryConfig {
    pub enabled: bool,
    pub log_file: String,
    pub batch_size: usize,
}

impl TelemetryManager {
    pub fn new(config: &TelemetryConfig) -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            config: config.clone(),
        }
    }

    pub async fn log_event(&self, event_type: &str, data: serde_json::Value) -> BotResult<()> {
        if !self.config.enabled {
            return Ok(());
        }

        let event = TelemetryEvent {
            timestamp: Utc::now(),
            event_type: event_type.to_string(),
            data,
        };

        let mut events = self.events.lock().await;
        events.push(event);

        if events.len() >= self.config.batch_size {
            self.flush_events().await?;
        }

        Ok(())
    }

    pub async fn log_command(&self, command_name: &str) -> BotResult<()> {
        self.log_event("command_used", json!({ "command": command_name })).await
    }

    pub async fn log_error(&self, error_type: &str, error_message: &str) -> BotResult<()> {
        self.log_event("error", json!({
            "type": error_type,
            "message": error_message
        })).await
    }

    pub async fn log_metric(&self, metric_name: &str, value: f64) -> BotResult<()> {
        self.log_event("metric", json!({
            "name": metric_name,
            "value": value
        })).await
    }

    async fn flush_events(&self) -> BotResult<()> {
        let mut events = self.events.lock().await;
        if events.is_empty() {
            return Ok(());
        }

        let events_json = serde_json::to_string(&events.drain(..).collect::<Vec<_>>())?;

        tokio::fs::append_to_file(&self.config.log_file, events_json.as_bytes()).await?;

        Ok(())
    }

    pub async fn start_periodic_flush(&self) {
        let telemetry_manager = self.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(300)).await;
                if let Err(e) = telemetry_manager.flush_events().await {
                    log::error!("Failed to flush telemetry events: {:?}", e);
                }
            }
        });
    }
}

impl Clone for TelemetryManager {
    fn clone(&self) -> Self {
        Self {
            events: Arc::clone(&self.events),
            config: self.config.clone(),
        }
    }
}

impl Clone for TelemetryConfig {
    fn clone(&self) -> Self {
        Self {
            enabled: self.enabled,
            log_file: self.log_file.clone(),
            batch_size: self.batch_size,
        }
    }
}