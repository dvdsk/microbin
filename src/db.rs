use std::todo;
use crate::error_handling::AppError;
use crate::pasta::Pasta;

pub trait Database {
    fn insert_pasta(&self, pasta: Pasta) -> Result<(), AppError>;
    fn find_all_pastas(&self) -> Result<Vec<Pasta>, AppError>;
    fn get_pasta(&self, id: &str) -> Result<Option<Pasta>, AppError>;
    fn update_pasta(&self, id: &str, pasta: Pasta) -> Result<Pasta, AppError>;
    fn find_all_public_pastas(&self) -> Result<Vec<Pasta>, AppError>;
}