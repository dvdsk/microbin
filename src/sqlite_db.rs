use sea_orm::{entity, EntityTrait};
use sea_orm::sqlx::SqlitePool;
use serde::de::Error;
use crate::db::Database;
use crate::error_handling::AppError;
use crate::pasta::Pasta;

use ::entity::{pasta::Entity as PastaEntiy};

pub struct SQLiteDB {
    connection: SqlitePool,
}

impl SQLiteDB {
    pub async fn new(path: &str) -> Self {
        let connection = SqlitePool::connect(&format!("{}/database.sqlite", path)).await.expect("Failed to connect to SQLite database");
        SQLiteDB { connection }
    }
}


impl Database for  SQLiteDB {
    async fn insert_pasta(&self, pasta: Pasta) -> Result<(), AppError> {
        Ok(PastaEntiy::insert(pasta.into()).exec(&self.connection.acquire().await.expect("")
            .as_ref()))
    }

    fn find_all_pastas(&self) -> Result<Vec<Pasta>, AppError> {

    }

    fn get_pasta(&self, id: &str) -> Result<Option<Pasta>, AppError> {
        todo!()
    }

    fn update_pasta(&self, id: &str, pasta: Pasta) -> Result<Pasta, AppError> {
        todo!()
    }

    fn find_all_public_pastas(&self) -> Result<Vec<Pasta>, AppError> {
        todo!()
    }
}