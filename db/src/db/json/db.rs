use std::path::PathBuf;
use crate::database::Database;
use crate::database_args::JSONDatabaseProperties;
use crate::db::error::DBError;
use crate::entities::pasta::PastaEntity;

pub struct JsonDatabase {
    pub path: String,
}

impl JsonDatabase {
    pub fn new(args: JSONDatabaseProperties) -> Result<Self, DBError> {
        let path_to_use = PathBuf::from(args.file_path).join(args.file_name).to_str().unwrap()
            .to_owned();
        std::fs::read_to_string(&path_to_use)?;
        Ok(Self {
            path: path_to_use,
        })
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
        Err(DBError::not_found(&format!("Pasta with id {} not found", id)))
    }

    fn find_all_public_pastas(&self) -> Result<Vec<crate::entities::pasta::PastaEntity>, crate::db::error::DBError> {
        let all_pastas = self.find_all_pastas()?;
        Ok(all_pastas.into_iter()
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