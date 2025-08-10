pub mod database;
pub mod database_args;


pub mod db {
    pub mod error;
    pub mod sqlite {
        pub mod db;
    }
    pub mod json {
        pub mod db;
    }
}

pub mod entities {
    pub mod pasta;
}

