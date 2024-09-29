use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub discord_id: i64,
    pub username: String,
    pub joined_at: DateTime<Utc>,
}

impl User {
    pub async fn create(pool: &sqlx::PgPool, discord_id: i64, username: &str) -> Result<Self, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (discord_id, username, joined_at) 
             VALUES ($1, $2, $3) 
             RETURNING id, discord_id, username, joined_at",
            discord_id,
            username,
            Utc::now()
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_by_discord_id(pool: &sqlx::PgPool, discord_id: i64) -> Result<Option<Self>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE discord_id = $1",
            discord_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }
}