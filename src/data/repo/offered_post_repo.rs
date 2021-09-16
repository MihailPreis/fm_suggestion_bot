use crate::data::model::offered_post::OfferedPost;
use crate::utils::result_utils::FatalValueMapper;
use sqlx::{Error, Pool, Sqlite};
use std::convert::TryInto;

#[derive(Clone)]
pub struct OfferedPostRepo {
    pool: Pool<Sqlite>,
}

impl OfferedPostRepo {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        OfferedPostRepo { pool }
    }

    pub async fn migrate(pool: &Pool<Sqlite>) {
        sqlx::migrate!()
            .run(pool)
            .await
            .map_value_or_exit("Can not migrate db!!!!".to_string());
    }

    pub async fn save_offered_post(&self, offered_post: OfferedPost) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO offered_post (message_id, chat_id, admin_chat_id, admin_chat_message_id) VALUES (?, ?, ?, ?)",
            offered_post.message_id,
            offered_post.chat_id,
            offered_post.admin_chat_id,
            offered_post.admin_chat_message_id,
        )
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn get_offered_post(
        &self,
        admin_chat_id: i64,
        admin_chat_message_id: i32,
    ) -> Result<OfferedPost, Error> {
        let result = sqlx::query!(
            "SELECT * FROM offered_post WHERE admin_chat_id = ? AND admin_chat_message_id = ?",
            admin_chat_id,
            admin_chat_message_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(OfferedPost::new(
            result.chat_id,
            result.message_id.try_into().unwrap(),
            result.admin_chat_id,
            result.admin_chat_message_id.try_into().unwrap(),
        ))
    }
}
