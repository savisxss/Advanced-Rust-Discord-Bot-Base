use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use crate::bot::error::{BotResult, BotError};

pub mod models;

pub struct Database {
    pool: Arc<PgPool>,
}

impl Database {
    pub async fn new(database_url: &str) -> BotResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .map_err(|e| BotError::Database(e.to_string()))?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub async fn execute_query(&self, query: &str) -> BotResult<()> {
        sqlx::query(query)
            .execute(&*self.pool)
            .await
            .map_err(|e| BotError::Database(e.to_string()))?;
        Ok(())
    }

    pub async fn run_migrations(&self) -> BotResult<()> {
        sqlx::migrate!("./migrations")
            .run(&*self.pool)
            .await
            .map_err(|e| BotError::Database(format!("Migration error: {}", e)))?;
        Ok(())
    }

    pub fn get_pool(&self) -> Arc<PgPool> {
        Arc::clone(&self.pool)
    }
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            pool: Arc::clone(&self.pool),
        }
    }
}