use tokio::task::JoinHandle;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::bot::error::BotResult;

pub struct TaskManager {
    tasks: Arc<Mutex<HashMap<String, JoinHandle<()>>>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn spawn<F>(&self, name: &str, future: F) -> BotResult<()>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let mut tasks = self.tasks.lock().await;
        if tasks.contains_key(name) {
            return Err(BotError::Internal(format!("Task '{}' already exists", name)));
        }
        let handle = tokio::spawn(future);
        tasks.insert(name.to_string(), handle);
        Ok(())
    }

    pub async fn cancel(&self, name: &str) -> BotResult<()> {
        let mut tasks = self.tasks.lock().await;
        if let Some(handle) = tasks.remove(name) {
            handle.abort();
            Ok(())
        } else {
            Err(BotError::Internal(format!("Task '{}' not found", name)))
        }
    }

    pub async fn cancel_all(&self) {
        let mut tasks = self.tasks.lock().await;
        for (_, handle) in tasks.drain() {
            handle.abort();
        }
    }
}