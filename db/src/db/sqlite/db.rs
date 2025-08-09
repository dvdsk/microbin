use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
pub use crate::database::{Database};
use crate::database_args::SqliteProperties;
use crate::db::error::DBError;
use crate::entities::pasta::PastaEntity;

pub struct SqLite {
    pub pool: r2d2::Pool<SqliteConnectionManager>,
}


pub static INSERT_QUERY: &str = r#"INSERT INTO pasta (
                id,
                content,
                file_name,
                file_size,
                extension,
                private,
                read_only,
                editable,
                encrypt_server,
                encrypt_client,
                encrypted_key,
                created,
                expiration,
                last_read,
                read_count,
                burn_after_reads,
                pasta_type
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)"#;


pub static CREATE_TABLE_QUERY: &str = r#"
        CREATE TABLE IF NOT EXISTS pasta (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            file_name TEXT,
            file_size INTEGER,
            extension TEXT NOT NULL,
            read_only INTEGER NOT NULL,
            private INTEGER NOT NULL,
            editable INTEGER NOT NULL,
            encrypt_server INTEGER NOT NULL,
            encrypt_client INTEGER NOT NULL,
            encrypted_key TEXT,
            created INTEGER NOT NULL,
            expiration INTEGER NOT NULL,
            last_read INTEGER NOT NULL,
            read_count INTEGER NOT NULL,
            burn_after_reads INTEGER NOT NULL,
            pasta_type TEXT NOT NULL,
            hide_read_count INTEGER NOT NULL
        );"#;


pub static SELECT_ALL_QUERY: &str = r#"SELECT * FROM pasta"#;

impl SqLite {
    pub fn new(args: SqliteProperties) -> Result<Self, rusqlite::Error> {

        let manager = if args.in_memory {
            SqliteConnectionManager::memory()
        } else {
            SqliteConnectionManager::file(args.db_path)
        };


        let pool = r2d2::Pool::new(manager).expect("db pool");
        let conn = pool.get().expect("should get connection from pool");
        conn.execute(
            CREATE_TABLE_QUERY,
        []
        )?;
        Ok(SqLite{
            pool,
        })
    }
}

impl Database for SqLite {
    fn insert_pasta(&self, pasta: PastaEntity) -> Result<(), DBError> {
        self.pool.get().expect("should get connection from pool").execute(
            INSERT_QUERY,
            params![
                pasta.id,
                pasta.content,
                pasta.file_name,
                pasta.file_size,
                pasta.extension,
                pasta.private,
                pasta.read_only,
                pasta.editable,
                pasta.encrypt_server,
                pasta.encrypt_client,
                pasta.encrypted_key,
                pasta.created,
                pasta.expiration,
                pasta.last_read,
                pasta.read_count,
                pasta.burn_after_reads,
                pasta.pasta_type
            ],
        )?;

        Ok(())
    }

    fn find_all_pastas(&self) -> Result<Vec<PastaEntity>, DBError> {
        let pool = self.pool.get().expect("should get connection from pool");

           let mut stmt = pool.prepare("SELECT * FROM pasta ORDER BY created ASC")?;
        let pasta_iter = stmt
            .query_map([], |row| {
                Ok(PastaEntity::from(row))
            }).expect("Failed to query pastas");

        Ok(pasta_iter
            .map(|r| r.expect("Failed to get pasta"))
            .collect::<Vec<PastaEntity>>())
    }

    fn get_pasta(&self, id: &u64) -> Result<Option<PastaEntity>, DBError> {
        let mut pool = self.pool.get().expect("should get connection from pool");
        let mut stmt = pool.prepare("SELECT\
         * FROM pasta WHERE id = ?1")?;
        let result_from_query = stmt.query_one(params![id], |row| {
            Ok(PastaEntity::from(row))
        });
        return match result_from_query {
            Ok(pasta)=> {
                Ok(Some(pasta))
            },
            Err(e)=> {
                match e {
                    rusqlite::Error::QueryReturnedNoRows => {
                        Ok(None)
                    },
                    _ => {
                        Err(DBError::from(e))
                    }
                }
            }
        }
    }

    fn update_pasta(&self, id: &u64, pasta: PastaEntity) -> Result<PastaEntity, DBError> {
        let mut pool = self.pool.get().expect("should get connection from pool");
        let mut stmt = pool.prepare(
            "UPDATE pasta SET content = ?1, file_name = ?2, file_size = ?3, extension = ?4, private = ?5, read_only = ?6, editable = ?7, encrypt_server = ?8, encrypt_client = ?9, encrypted_key = ?10, created = ?11, expiration = ?12, last_read = ?13, read_count = ?14, burn_after_reads = ?15, pasta_type = ?16 WHERE id = ?17"
        )?;

        stmt.execute(params![
            pasta.content,
            pasta.file_name,
            pasta.file_size,
            pasta.extension,
            pasta.private,
            pasta.read_only,
            pasta.editable,
            pasta.encrypt_server,
            pasta.encrypt_client,
            pasta.encrypted_key,
            pasta.created,
            pasta.expiration,
            pasta.last_read,
            pasta.read_count,
            pasta.burn_after_reads,
            pasta.pasta_type,
            id
        ])?;

        Ok(pasta)
    }

    fn find_all_public_pastas(&self) -> Result<Vec<PastaEntity>, DBError> {
        let mut pool = self.pool.get().expect("should get connection from pool");
            let mut stmt = pool.prepare("SELECT * FROM pasta WHERE private = 0 ORDER BY created \
            ASC")?;
        let pasta_iter = stmt
            .query_map([], |row| {
                Ok(PastaEntity::from(row))
            }).expect("Failed to query public pastas");

        Ok(pasta_iter
            .map(|r| r.expect("Failed to get public pasta"))
            .collect::<Vec<PastaEntity>>())
    }

    fn delete_pasta(&self, id: &u64) -> Result<(), DBError> {
        let mut pool = self.pool.get().expect("should get connection from pool");
        let mut stmt = pool.prepare("DELETE  FROM pasta WHERE id = ?1")?;
        stmt.execute(params![id])?;
        Ok(())
    }
}