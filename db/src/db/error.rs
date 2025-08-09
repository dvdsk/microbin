use rusqlite::Error;
use std::fmt::Debug;

pub struct DBError {
    pub message: String,
}

impl DBError {
    pub fn parse_error(p0: &str) -> Self {
        DBError {
            message: p0.to_string(),
        }
    }

    pub fn not_found(p0: &str) -> Self {
        DBError {
            message: format!("Not found: {}", p0),
        }
    }
}

impl From<std::io::Error> for DBError {
    fn from(value: std::io::Error) -> Self {
        DBError {
            message: value.to_string(),
        }
    }
}

impl std::fmt::Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DBError: {}", self.message)
    }
}

impl From<serde_json::Error> for DBError {
    fn from(value: serde_json::Error) -> Self {
        DBError {
            message: value.to_string(),
        }
    }
}

impl Debug for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DBError: {}", self.message)
    }
}

impl From<Error> for DBError {
    fn from(value: Error) -> Self {
        DBError {
            message: value.to_string(),
        }
    }
}
