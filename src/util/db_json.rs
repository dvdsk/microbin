use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Once;

use serde_json::Value;

use crate::Pasta;

fn database_path() -> &'static Path {
    Path::new("pasta_data/database.json")
}

pub fn read_all() -> Vec<Pasta> {
    load_from_file(database_path()).expect("Failed to load pastas from JSON")
}

pub fn update_all(pastas: &Vec<Pasta>) {
    save_to_file(database_path(), pastas);
}

fn save_to_file(path: &Path, pasta_data: &Vec<Pasta>) {
    // This uses a two stage write. First we write to a new file, if this fails
    // only the new pasta's are lost. Then we replace the current database with
    // the new file. This either succeeds or fails. The database is never left
    // in an undefined state.
    let tmp_file_path = path.with_extension(".tmp");
    let tmp_file = File::create(&tmp_file_path).expect(&format!(
        "failed to create temporary database file for writing. path: {}",
        tmp_file_path.display()
    ));

    let writer = BufWriter::new(tmp_file);
    serde_json::to_writer(writer, &pasta_data)
        .expect("Should be able to write out data to database file");
    std::fs::rename(tmp_file_path, path).expect("Could not update database");
}

fn migrate(path: &Path) {
    let Ok(file) = File::open(path) else {
        return;
    };

    let reader = BufReader::new(file);
    let mut partially_deserialized: Value = serde_json::from_reader(reader).unwrap();
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
    save_to_file(path, &pasta_data)
}

fn load_from_file(path: &Path) -> io::Result<Vec<Pasta>> {
    static INIT_JSON_DB: Once = Once::new();
    INIT_JSON_DB.call_once(|| {
        // lets not migrate every read
        // read happens before any update therefore
        // its safe to only migrate here
        migrate(path);
    });

    let file = File::open(path);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let data: Vec<Pasta> = match serde_json::from_reader(reader) {
                Ok(t) => t,
                _ => Vec::new(),
            };
            Ok(data)
        }
        Err(_) => {
            log::info!("Database file {} not found!", path.display());
            save_to_file(path, &Vec::<Pasta>::new());

            log::info!("Database file {} created.", path.display());
            load_from_file(path)
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

    #[test]
    fn test_migration() {
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

        let migrated_db = load_from_file(tmpfile.path()).unwrap();
        assert_eq!(migrated_db[0].hide_read_count, false);
    }
}
