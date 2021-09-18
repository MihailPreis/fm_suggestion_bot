use sqlx::{Error, Pool, Sqlite};

use crate::data::model::cached_pic::CachedPic;

#[derive(Clone)]
pub struct CachedPicRepo {
    pool: Pool<Sqlite>,
}

impl CachedPicRepo {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        CachedPicRepo { pool }
    }

    pub async fn save_cached_pic(&self, cached_pic: CachedPic) -> Result<(), Error> {
        sqlx::query!(
            "INSERT INTO cached_pic (image_name, image_file_id) VALUES (?, ?)",
            cached_pic.image_name,
            cached_pic.image_file_id,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_cached_pic(&self, image_name: String) -> Result<CachedPic, Error> {
        let result = sqlx::query!("SELECT * FROM cached_pic WHERE image_name = ?", image_name)
            .fetch_one(&self.pool)
            .await?;
        Ok(CachedPic::new(result.image_name, result.image_file_id))
    }
}
