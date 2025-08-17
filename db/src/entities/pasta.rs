use rusqlite::Row;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct PastaEntity {
    pub id: u64,
    pub content: String,
    pub file_name: Option<String>,
    pub file_size: Option<i64>,
    pub extension: String,
    pub read_only: bool,
    pub private: bool,
    pub editable: i32,
    pub encrypt_server: i32,
    pub encrypt_client: i32,
    pub encrypted_key: Option<String>,
    pub created: i64,
    pub expiration: i64,
    pub last_read: i64,
    pub read_count: i64,
    pub burn_after_reads: i64,
    pub pasta_type: String,
    pub hide_read_count: bool,
}

impl From<&Row<'_>> for PastaEntity {
    fn from(row: &Row<'_>) -> Self {
        PastaEntity {
            id: row.get(0).unwrap(),
            content: row.get(1).unwrap(),
            file_name: row.get(2).unwrap(),
            file_size: row.get(3).unwrap(),
            extension: row.get(4).unwrap(),
            read_only: row.get(5).unwrap(),
            private: row.get(6).unwrap(),
            editable: row.get(7).unwrap(),
            encrypt_server: row.get(8).unwrap(),
            encrypt_client: row.get(9).unwrap(),
            encrypted_key: row.get(10).unwrap(),
            created: row.get(11).unwrap(),
            expiration: row.get(12).unwrap(),
            last_read: row.get(13).unwrap(),
            read_count: row.get(14).unwrap(),
            burn_after_reads: row.get(15).unwrap(),
            pasta_type: row.get(16).unwrap(),
            hide_read_count: row.get(17).unwrap(),
        }
    }
}
