use sqlx::{Error, Pool, Sqlite};

use crate::data::model::pic::Pic;

#[derive(Clone)]
pub struct PicRepo {
    pool: Pool<Sqlite>,
}

impl PicRepo {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        PicRepo { pool }
    }

    pub async fn save_pic(&self, pic: Pic) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO pic (file_name, for_accept, data) VALUES (?, ?, ?)",
            pic.file_name,
            pic.for_accept,
            pic.data,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_pic(&self, file_name: String, for_accept: bool) -> Result<(), Error> {
        sqlx::query!(
            "DELETE FROM pic WHERE (file_name, for_accept) IN
            (SELECT file_name, for_accept FROM pic WHERE file_name == ? AND for_accept == ?)",
            file_name,
            for_accept
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_all_pics(&self) -> Result<Vec<Pic>, Error> {
        Ok(
            sqlx::query_as!(Pic, "SELECT file_name, for_accept, data FROM pic")
                .fetch_all(&self.pool)
                .await?,
        )
    }

    pub async fn get_pic(&self, file_name: String, for_accept: bool) -> Result<Pic, Error> {
        Ok(sqlx::query_as!(
            Pic,
            "SELECT file_name, for_accept, data FROM pic WHERE file_name = ? AND for_accept = ?",
            file_name,
            for_accept
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn get_random_pic(&self, for_accept: bool) -> Result<Pic, Error> {
        Ok(sqlx::query_as!(
            Pic,
            "SELECT file_name, for_accept, data FROM pic WHERE for_accept = ? ORDER BY RANDOM() LIMIT 1",
            for_accept
        )
        .fetch_one(&self.pool)
        .await?)
    }
}
