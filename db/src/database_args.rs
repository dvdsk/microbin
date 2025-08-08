#[derive(Debug)]
pub enum DatabaseArgs {
    SqliteProperties(SqliteProperties),
    JSONDatabaseProperties(JSONDatabaseProperties),
}

#[derive(Debug)]
pub struct SqliteProperties {
    pub db_path: String,
    pub in_memory: bool,
}

#[derive(Debug)]
pub struct JSONDatabaseProperties {
    pub file_path: String,
    pub file_name: String,
}