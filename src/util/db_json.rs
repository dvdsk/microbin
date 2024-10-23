use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::fs::{self, File};

use serde_json::Value;
use tokio::io::AsyncWriteExt;

use crate::Pasta;

fn database_path() -> &'static Path {
    Path::new("pasta_data/database.json")
}

pub async fn read_all() -> Vec<Pasta> {
    load_from_file(database_path())
        .await
        .expect("Failed to load pastas from JSON")
}

pub async fn update_all(pastas: &Vec<Pasta>) {
    save_to_file(database_path(), pastas).await;
}

async fn save_to_file(path: &Path, pasta_data: &Vec<Pasta>) {
    // This uses a two stage write. First we write to a new file, if this fails
    // only the new pasta's are lost. Then we replace the current database with
    // the new file. This either succeeds or fails. The database is never left
    // in an undefined state.
    let tmp_file_path = path.with_extension(".tmp");
    let mut tmp_file = File::create(&tmp_file_path).await.expect(&format!(
        "failed to create temporary database file for writing. path: {}",
        tmp_file_path.display()
    ));

    let encoded =
        serde_json::to_vec(&pasta_data).expect("Should be able to write out data to database file");
    // TODO report this error, actually return errors from all functions
    // and nicely report them in the caller instead of sprinkling
    // log::error everywhere.
    tmp_file.write_all(&encoded).await.unwrap();
    fs::rename(tmp_file_path, path)
        .await
        .expect("Could not update database");
}

async fn migrate(path: &Path) {
    let serialized = fs::read(path).await.expect("file should exist");
    let mut partially_deserialized: Value = serde_json::from_slice(&serialized).unwrap();
    let data = partially_deserialized
        .as_array_mut()
        .expect("should be vec");

    for pasta in data {
        let pasta = pasta.as_object_mut().expect("should be pasta struct");
        // add migrations here
        pasta.entry("hide_read_count").or_insert(Value::Bool(false));
    }
    let pasta_data: Vec<Pasta> =
        serde_json::from_value(partially_deserialized).expect("missing fields where added");
    save_to_file(path, &pasta_data).await
}

async fn load_from_file(path: &Path) -> io::Result<Vec<Pasta>> {
    static NOT_YET_MIGRATED: AtomicBool = AtomicBool::new(true);
    if NOT_YET_MIGRATED.load(Ordering::Relaxed) {
        migrate(path).await;
    }

    match fs::read(path).await {
        Ok(serialized) => {
            let data: Vec<Pasta> = match serde_json::from_slice(&serialized) {
                Ok(t) => t,
                Err(e) => panic!("Database file corrupted, deserialize error: {e:?}"),
            };
            Ok(data)
        }
        Err(_) => {
            log::info!("Database file {} not found!", path.display());
            save_to_file(path, &Vec::<Pasta>::new()).await;
            log::info!("Database file {} created.", path.display());
            Ok(Vec::new())
        }
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};
    use tempfile::NamedTempFile;

    use crate::pasta::PastaFile;

    use super::*;
    use std::io::Write;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct OldPasta {
        pub id: u64,
        pub content: String,
        pub file: Option<PastaFile>,
        pub extension: String,
        pub private: bool,
        pub readonly: bool,
        pub editable: bool,
        pub encrypt_server: bool,
        pub encrypt_client: bool,
        pub encrypted_key: Option<String>,
        pub created: i64,
        pub expiration: i64,
        pub last_read: i64,
        pub read_count: u64,
        pub burn_after_reads: u64,
        pub pasta_type: String,
    }

    #[tokio::test]
    async fn test_migration() {
        let mut tmpfile = NamedTempFile::new().unwrap();

        let old_db = vec![OldPasta {
            id: 1,
            content: "test content".to_string(),
            file: None,
            extension: "test".to_string(),
            private: false,
            readonly: false,
            editable: false,
            encrypt_server: false,
            encrypt_client: false,
            encrypted_key: None,
            created: 42,
            expiration: 42,
            last_read: 42,
            read_count: 42,
            burn_after_reads: 42,
            pasta_type: "text".to_string(),
        }];

        tmpfile
            .write(&serde_json::to_vec(&old_db).unwrap())
            .unwrap();

        let migrated_db = load_from_file(tmpfile.path()).await.unwrap();
        assert_eq!(migrated_db[0].hide_read_count, false);
    }
}
