use sqlx::{Error, Pool, Sqlite};

use crate::data::model::ban::Ban;

#[derive(Clone)]
pub struct BanRepo {
    pool: Pool<Sqlite>,
}

impl BanRepo {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        BanRepo { pool }
    }

    pub async fn create(&self, chat_id: i64, user_name: String, date: String) -> Result<(), Error> {
        sqlx::query!(
            "INSERT OR IGNORE INTO bans(chat_id, user_name, date, is_ban) VALUES (?, ?, ?, ?)",
            chat_id,
            user_name,
            date,
            false,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_list(&self) -> Result<Vec<Ban>, Error> {
        Ok(
            sqlx::query_as!(Ban, "SELECT * FROM bans WHERE is_ban = 1")
                .fetch_all(&self.pool)
                .await?,
        )
    }

    pub async fn is_banned(&self, chat_id: i64) -> Result<bool, Error> {
        let result = sqlx::query!("SELECT * FROM bans WHERE chat_id = ?", chat_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(result.is_ban)
    }

    pub async fn update(&self, chat_id: i64, date: String, is_ban: bool) -> Result<(), Error> {
        sqlx::query!(
            "UPDATE bans SET is_ban = ?, date = ? WHERE chat_id = ?",
            is_ban,
            date,
            chat_id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
