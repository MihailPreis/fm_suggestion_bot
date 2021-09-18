use lazy_static::lazy_static;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Pool, Sqlite};

use crate::utils::env_utils::get_env_key;
use crate::utils::result_utils::FatalValueMapper;

static DB_URL_KEY: &str = "DATABASE_URL";

lazy_static! {
    static ref DB_URL: String = get_env_key(DB_URL_KEY);
}

pub async fn create_pool() -> Pool<Sqlite> {
    Pool::connect(&**DB_URL)
        .await
        .map_value_or_exit("Can not connect to db".to_string())
}

pub async fn create_database_if_needed() {
    if !sqlx::Sqlite::database_exists(&**DB_URL).await.unwrap() {
        sqlx::Sqlite::create_database(&**DB_URL)
            .await
            .map_value_or_exit("Can not create db!!".to_string());
    }
}

pub async fn migrate(pool: &Pool<Sqlite>) {
    sqlx::migrate!()
        .run(pool)
        .await
        .map_value_or_exit("Can not migrate db!!!!".to_string());
}
