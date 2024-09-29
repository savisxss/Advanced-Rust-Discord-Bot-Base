use tokio::task::JoinHandle;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::bot::error::{BotResult, BotError};
use std::future::Future;

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

    pub async fn is_running(&self, name: &str) -> bool {
        let tasks = self.tasks.lock().await;
        tasks.contains_key(name)
    }

    pub async fn get_running_tasks(&self) -> Vec<String> {
        let tasks = self.tasks.lock().await;
        tasks.keys().cloned().collect()
    }
}

impl Drop for TaskManager {
    fn drop(&mut self) {
        let tasks = self.tasks.clone();
        tokio::spawn(async move {
            let mut tasks = tasks.lock().await;
            for (_, handle) in tasks.drain() {
                handle.abort();
            }
        });
    }
}