use crate::db::json::db::JsonDatabase;
use crate::database_args::DatabaseArgs;
use crate::db::error::DBError;
use crate::db::sqlite::db::SqLite;
use crate::entities::pasta::PastaEntity;

pub trait Database {
    fn insert_pasta(&self, pasta: PastaEntity) -> Result<(), DBError>;
    fn find_all_pastas(&self) -> Result<Vec<PastaEntity>, DBError>;
    fn get_pasta(&self, id: &u64) -> Result<Option<PastaEntity>, DBError>;
    fn update_pasta(&self, id: &u64, pasta: PastaEntity) -> Result<PastaEntity, DBError>;
    fn find_all_public_pastas(&self) -> Result<Vec<PastaEntity>, DBError>;
    fn delete_pasta(&self, id: &u64) -> Result<(), DBError>;
}

pub type DatabaseType = Box<dyn Database + Send + Sync>;

pub fn get_database(args: DatabaseArgs) -> DatabaseType {
    match args {
        DatabaseArgs::SqliteProperties(sqlite_props) => {
            let db = SqLite::new(sqlite_props).expect("Error initializing SQLite database");
            Box::new(db)
        }
        DatabaseArgs::JSONDatabaseProperties(json_database_props) => {
            let db = JsonDatabase::new(json_database_props).expect(
                "Error initializing Json \
            database ",
            );
            Box::new(db)
        }
    }
}
