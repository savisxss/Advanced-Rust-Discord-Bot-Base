use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;

pub struct Database {
    pool: Arc<PgPool>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub async fn execute_query(&self, query: &str) -> Result<(), sqlx::Error> {
        sqlx::query(query).execute(&*self.pool).await?;
        Ok(())
    }
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            pool: Arc::clone(&self.pool),
        }
    }
}