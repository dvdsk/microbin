use crate::database::Database;
use crate::database_args::JSONDatabaseProperties;
use crate::db::error::DBError;
use crate::entities::pasta::PastaEntity;
use std::path::PathBuf;

pub struct JsonDatabase {
    pub path: String,
}

impl JsonDatabase {
    pub fn new(args: JSONDatabaseProperties) -> Result<Self, DBError> {
        let path_to_use = PathBuf::from(args.file_path)
            .join(args.file_name)
            .to_str()
            .unwrap()
            .to_owned();
        std::fs::read_to_string(&path_to_use)?;
        Ok(Self { path: path_to_use })
    }
    pub fn read_json(&self) -> Result<Vec<PastaEntity>, crate::db::error::DBError> {
        let data = std::fs::read_to_string(&self.path)?;
        serde_json::from_str::<Vec<PastaEntity>>(&data)
            .map_err(|e| DBError::parse_error(&format!("Failed to parse JSON: {}", e)))
    }

    pub fn write_json(&self, pastas: &Vec<PastaEntity>) -> Result<(), crate::db::error::DBError> {
        let data = serde_json::to_string(pastas)?;
        std::fs::write(&self.path, &data)?;
        Ok(())
    }
}

impl Database for JsonDatabase {
    fn insert_pasta(&self, pasta: PastaEntity) -> Result<(), DBError> {
        let mut current_pastas = self.read_json()?;
        current_pastas.push(pasta);
        self.write_json(&current_pastas)?;
        Ok(())
    }

    fn find_all_pastas(&self) -> Result<Vec<PastaEntity>, DBError> {
        let pastas = self.read_json()?;
        Ok(pastas)
    }

    fn get_pasta(&self, _id: &u64) -> Result<Option<PastaEntity>, DBError> {
        let pastas = self.read_json()?;
        for pasta in pastas {
            if pasta.id == *_id {
                return Ok(Some(pasta));
            }
        }
        Ok(None)
    }

    fn update_pasta(&self, id: &u64, pasta_updated: PastaEntity) -> Result<PastaEntity, DBError> {
        let mut pastas = self.read_json()?;
        for pasta in pastas.iter_mut() {
            if pasta.id == *id {
                *pasta = pasta_updated;
                let cloned_pasta = pasta.clone();
                self.write_json(&pastas)?;
                return Ok(cloned_pasta);
            }
        }
        Err(DBError::not_found(&format!(
            "Pasta with id {} not found",
            id
        )))
    }

    fn find_all_public_pastas(
        &self,
    ) -> Result<Vec<crate::entities::pasta::PastaEntity>, crate::db::error::DBError> {
        let all_pastas = self.find_all_pastas()?;
        Ok(all_pastas
            .into_iter()
            .filter(|pasta| !pasta.private)
            .collect())
    }

    fn delete_pasta(&self, id: &u64) -> Result<(), DBError> {
        let mut pastas = self.read_json()?;
        pastas.retain(|pasta| pasta.id != *id);
        self.write_json(&pastas)?;
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_insert_pasta() {
        let db = JsonDatabase::new(super::super::super::test_utils::test_util::create_test_json_db_properties()).expect("Failed to \
        create \
        Json database");

        let pasta = super::super::super::test_utils::test_util::create_random_pasta_entity();

        db.insert_pasta(pasta.clone())
            .expect("Failed to insert pasta");
        let pastas = db.find_all_pastas().expect("Failed to find all pastas");
        assert_eq!(pastas.len(), 1);

        let retrieved_pasta = db.get_pasta(&pasta.id).expect("Failed to get pasta");
        assert!(retrieved_pasta.is_some());
    }

    #[test]
    fn test_create_and_insert_update_pasta() {
        let db = JsonDatabase::new(super::super::super::test_utils::test_util::create_test_json_db_properties()).expect("Failed to \
        create \
        Json database");
        let pasta = super::super::super::test_utils::test_util::create_random_pasta_entity();

        db.insert_pasta(pasta.clone())
            .expect("Failed to insert pasta");
        let pastas = db.find_all_pastas().expect("Failed to find all pastas");
        assert_eq!(pastas.len(), 1);

        let retrieved_pasta = db.get_pasta(&pasta.id).expect("Failed to get pasta");
        let retrieved_pasta = retrieved_pasta.expect("Pasta should exist");
        assert_eq!(retrieved_pasta.id, pasta.id);

        let updated_pasta = super::super::super::test_utils::test_util
        ::create_random_pasta_entity();
        db.update_pasta(&pasta.id, updated_pasta.clone())
            .expect("Failed to update pasta");
        let updated_retrieved_pasta = db.get_pasta(&pasta.id).expect("Failed to get pasta after update");
        let updated_retrieved_pasta = updated_retrieved_pasta.expect("Pasta should exist after update");
        assert_eq!(updated_retrieved_pasta.id, retrieved_pasta.id);
        assert_ne!(retrieved_pasta.content, updated_retrieved_pasta.content);
    }


    #[test]
    fn test_find_all_public_pastas() {
        let db = JsonDatabase::new(super::super::super::test_utils::test_util::create_test_json_db_properties()).expect("Failed to \
        create \
        Json database");
        let mut pasta1 = super::super::super::test_utils::test_util::create_random_pasta_entity();
        pasta1.private = true;
        let pasta2 = super::super::super::test_utils::test_util::create_random_pasta_entity();
        let pasta3 = super::super::super::test_utils::test_util::create_random_pasta_entity();

        db.insert_pasta(pasta1.clone())
            .expect("Failed to insert pasta1");
        db.insert_pasta(pasta2.clone())
            .expect("Failed to insert pasta2");
        db.insert_pasta(pasta3.clone())
            .expect("Failed to insert pasta3");

        let public_pastas = db.find_all_public_pastas().expect("Failed to find all public pastas");
        let all_pastas = db.find_all_pastas().expect("Failed to find all pastas");
        assert_eq!(public_pastas.len(), 2);
        assert!(public_pastas.iter().any(|p| p.id == pasta2.id));
        assert!(public_pastas.iter().any(|p| p.id == pasta3.id));
        assert_eq!(all_pastas.len(), 3);
        assert!(all_pastas.iter().any(|p| p.id == pasta1.id));
        assert!(all_pastas.iter().any(|p| p.id == pasta2.id));
        assert!(all_pastas.iter().any(|p| p.id == pasta3.id));
    }

    #[test]
    fn test_delete_pasta() {
        let db = JsonDatabase::new(super::super::super::test_utils::test_util::create_test_json_db_properties()).expect("Failed to \
        create \
        Json database");
        let pasta = super::super::super::test_utils::test_util::create_random_pasta_entity();

        db.insert_pasta(pasta.clone())
            .expect("Failed to insert pasta");
        let pastas = db.find_all_pastas().expect("Failed to find all pastas");
        assert_eq!(pastas.len(), 1);

        db.delete_pasta(&pasta.id).expect("Failed to delete pasta");
        let pastas_after_delete = db.find_all_pastas().expect("Failed to find all pastas after delete");
        assert_eq!(pastas_after_delete.len(), 0);
    }
}
