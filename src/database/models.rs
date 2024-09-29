use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(FromRow)]
pub struct User {
    pub id: i64,
    pub discord_id: i64,
    pub username: String,
    pub joined_at: DateTime<Utc>,
    pub experience: i32,
    pub level: i32,
}

#[derive(FromRow)]
pub struct Warning {
    pub id: i64,
    pub user_id: i64,
    pub moderator_id: i64,
    pub reason: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub async fn create(pool: &sqlx::PgPool, discord_id: i64, username: &str) -> Result<Self, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (discord_id, username, joined_at, experience, level) 
             VALUES ($1, $2, $3, $4, $5) 
             RETURNING id, discord_id, username, joined_at, experience, level",
            discord_id,
            username,
            Utc::now(),
            0,
            1
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

impl Warning {
    pub async fn create(pool: &sqlx::PgPool, user_id: i64, moderator_id: i64, reason: &str) -> Result<Self, sqlx::Error> {
        let warning = sqlx::query_as!(
            Warning,
            "INSERT INTO warnings (user_id, moderator_id, reason, created_at) 
             VALUES ($1, $2, $3, $4) 
             RETURNING id, user_id, moderator_id, reason, created_at",
            user_id,
            moderator_id,
            reason,
            Utc::now()
        )
        .fetch_one(pool)
        .await?;

        Ok(warning)
    }
}