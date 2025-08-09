#[cfg(test)]
pub mod test_util {

    const CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    use random_string::generate;

    pub fn create_test_sqlite_properties() -> super::super::super::database_args::SqliteProperties {
        super::super::super::database_args::SqliteProperties {
            db_path: ":memory:".to_string(),
            in_memory: true,
        }
    }

    pub fn create_test_json_db_properties()
    -> super::super::super::database_args::JSONDatabaseProperties {
        super::super::super::database_args::JSONDatabaseProperties {
            file_path: "./test_data".to_string(),
            file_name: format!("pasta_{}.json", generate(20, CHARSET)),
        }
    }

    pub fn create_random_pasta_entity() -> super::super::super::entities::pasta::PastaEntity {
        use rand::Rng;
        let mut rng = rand::rng();

        super::super::super::entities::pasta::PastaEntity {
            id: rng.random_range(0..100),
            content: generate(6, CHARSET),
            file_name: Option::from(generate(6, CHARSET)),
            file_size: Some(1024),
            extension: generate(6, CHARSET),
            private: false,
            read_only: false,
            editable: i32::from(true),
            encrypt_server: i32::from(false),
            encrypt_client: i32::from(false),
            encrypted_key: None,
            created: rng.random_range(0..1000000),
            expiration: 0,
            last_read: 0,
            read_count: 0,
            burn_after_reads: 0,
            pasta_type: "url".to_string(),
            hide_read_count: false,
        }
    }
}
