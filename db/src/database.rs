use crate::database_args::DatabaseArgs;
use crate::db::json::db::JsonDatabase;
use crate::db::sqlite::db::SqLite;
use crate::entities::pasta::PastaEntity;
use color_eyre::eyre;
use eyre::Result;
use std::sync::Arc;

pub trait Database {
    fn insert_pasta(&self, pasta: PastaEntity) -> Result<()>;
    fn find_all_pastas(&self) -> Result<Vec<PastaEntity>>;
    fn get_pasta(&self, id: &u64) -> Result<Option<PastaEntity>>;
    fn update_pasta(&self, id: &u64, pasta: PastaEntity) -> Result<PastaEntity>;
    fn find_all_public_pastas(&self) -> Result<Vec<PastaEntity>>;
    fn delete_pasta(&self, id: &u64) -> Result<()>;
}

pub type DatabaseType = Arc<dyn Database + Send + Sync>;

pub fn get_database(args: DatabaseArgs) -> DatabaseType {
    match args {
        DatabaseArgs::SqliteProperties(sqlite_props) => {
            let db = SqLite::new(sqlite_props).expect("Error initializing SQLite database");
            Arc::new(db)
        }
        DatabaseArgs::JSONDatabaseProperties(json_database_props) => {
            let db = JsonDatabase::new(json_database_props).expect(
                "Error initializing Json \
            database ",
            );
            Arc::new(db)
        }
    }
}
