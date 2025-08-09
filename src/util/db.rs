use crate::args::Args;
use crate::pasta::Pasta;

#[cfg(not(feature = "default"))]
const PANIC_MSG: &'static str = "Can not run without argument json-db, this version of microbin was compiled without rusqlite support. Make sure you do not pass in no-default-features during compilation";

#[cfg(feature = "default")]
pub fn read_all(args: &Args) -> Vec<Pasta> {
    if args.json_db {
        super::db_json::read_all()
    } else {
        super::db_sqlite::read_all(args)
    }
}

#[cfg(not(feature = "default"))]
pub fn read_all(args: Args) -> Vec<Pasta> {
    if args.json_db {
        super::db_json::read_all()
    } else {
        panic!("{}", PANIC_MSG);
    }
}

#[allow(unused)]
pub fn insert(pastas: Option<&Vec<Pasta>>, pasta: Option<&Pasta>, args: &Args) {
    if args.json_db {
        super::db_json::update_all(pastas.expect("Called insert() without passing Pasta vector"));
    } else {
        #[cfg(feature = "default")]
        super::db_sqlite::insert(
            pasta.expect("Called insert() without passing new Pasta"),
            args,
        );
        #[cfg(not(feature = "default"))]
        panic!();
    }
}

#[allow(unused)]
pub fn update(pastas: Option<&Vec<Pasta>>, pasta: Option<&Pasta>, args: &Args) {
    if args.json_db {
        super::db_json::update_all(pastas.expect("Called update() without passing Pasta vector"));
    } else {
        #[cfg(feature = "default")]
        super::db_sqlite::update(
            pasta.expect("Called insert() without passing Pasta to update"),
            args,
        );
        #[cfg(not(feature = "default"))]
        panic!("{}", PANIC_MSG);
    }
}

#[allow(unused)]
pub fn delete(pastas: Option<&Vec<Pasta>>, id: Option<u64>, args: &Args) {
    if args.json_db {
        super::db_json::update_all(pastas.expect("Called delete() without passing Pasta vector"));
    } else {
        #[cfg(feature = "default")]
        super::db_sqlite::delete_by_id(id.expect("Called delete() without passing Pasta id"), args);
        #[cfg(not(feature = "default"))]
        panic!("{}", PANIC_MSG);
    }
}
