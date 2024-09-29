use tokio::task::JoinHandle;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::bot::error::{BotResult, BotError};
use std::future::Future;

pub struct TaskManager {
    tasks: Arc<Mutex<HashMap<String, JoinHandle<()>>>>,
    queue: Arc<Mutex<VecDeque<(String, Box<dyn Future<Output = ()> + Send + 'static>)>>>,
    max_concurrent_tasks: usize,
}

impl TaskManager {
    pub fn new(max_concurrent_tasks: usize) -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            queue: Arc::new(Mutex::new(VecDeque::new())),
            max_concurrent_tasks,
        }
    }

    pub async fn spawn<F>(&self, name: &str, future: F) -> BotResult<()>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let mut tasks = self.tasks.lock().await;
        if tasks.len() >= self.max_concurrent_tasks {
            let mut queue = self.queue.lock().await;
            queue.push_back((name.to_string(), Box::new(future)));
            Ok(())
        } else {
            self.spawn_task(name, future, &mut tasks).await
        }
    }

    async fn spawn_task<F>(&self, name: &str, future: F, tasks: &mut HashMap<String, JoinHandle<()>>) -> BotResult<()>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        if tasks.contains_key(name) {
            return Err(BotError::Internal(format!("Task '{}' already exists", name)));
        }
        let queue = self.queue.clone();
        let tasks_clone = self.tasks.clone();
        let name_clone = name.to_string();
        let handle = tokio::spawn(async move {
            future.await;
            Self::task_completed(name_clone, queue, tasks_clone).await;
        });
        tasks.insert(name.to_string(), handle);
        Ok(())
    }

    async fn task_completed(name: String, queue: Arc<Mutex<VecDeque<(String, Box<dyn Future<Output = ()> + Send + 'static>)>>>, tasks: Arc<Mutex<HashMap<String, JoinHandle<()>>>>) {
        let mut tasks = tasks.lock().await;
        tasks.remove(&name);
        
        let mut queue = queue.lock().await;
        if let Some((next_name, next_future)) = queue.pop_front() {
            let _ = Self::spawn_task(&next_name, next_future, &mut tasks).await;
        }
    }

    pub async fn cancel(&self, name: &str) -> BotResult<()> {
        let mut tasks = self.tasks.lock().await;
        if let Some(handle) = tasks.remove(name) {
            handle.abort();
            Ok(())
        } else {
            let mut queue = self.queue.lock().await;
            if let Some(index) = queue.iter().position(|(n, _)| n == name) {
                queue.remove(index);
                Ok(())
            } else {
                Err(BotError::Internal(format!("Task '{}' not found", name)))
            }
        }
    }

    pub async fn cancel_all(&self) {
        let mut tasks = self.tasks.lock().await;
        for (_, handle) in tasks.drain() {
            handle.abort();
        }
        let mut queue = self.queue.lock().await;
        queue.clear();
    }

    pub async fn is_running(&self, name: &str) -> bool {
        let tasks = self.tasks.lock().await;
        tasks.contains_key(name) || self.queue.lock().await.iter().any(|(n, _)| n == name)
    }

    pub async fn get_running_tasks(&self) -> Vec<String> {
        let tasks = self.tasks.lock().await;
        let queue = self.queue.lock().await;
        let mut running_tasks: Vec<String> = tasks.keys().cloned().collect();
        running_tasks.extend(queue.iter().map(|(n, _)| n.clone()));
        running_tasks
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