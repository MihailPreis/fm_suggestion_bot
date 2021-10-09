use crate::data::model::stats::UserStats;
use sqlx::{Error, Pool, Sqlite};

#[derive(Clone)]
pub struct StatsRepo {
    pool: Pool<Sqlite>,
}

impl StatsRepo {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    pub async fn get_stat_for_user(&self, user_id: i64) -> Option<UserStats> {
        sqlx::query_as!(
            UserStats,
            "SELECT * FROM user_stats WHERE user_id = ?",
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .ok()
    }

    pub async fn get_stat_for_user_or_default(&self, user_id: i64) -> UserStats {
        self.get_stat_for_user(user_id)
            .await
            .unwrap_or_else(|| UserStats::empty(user_id))
    }

    pub async fn increment_offered(&self, user_id: i64) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO user_stats
VALUES (?, 1, 0, 0)
ON CONFLICT (user_id) DO UPDATE SET offered_count = offered_count + 1",
            user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn increment_accepted(&self, user_id: i64) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO user_stats
VALUES (?, 1, 1, 0)
ON CONFLICT (user_id) DO UPDATE SET accepted_count = accepted_count + 1",
            user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn increment_declined(&self, user_id: i64) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO user_stats
VALUES (?, 1, 0, 1)
ON CONFLICT (user_id) DO UPDATE SET declined_count = declined_count + 1",
            user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
