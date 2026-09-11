use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use rusqlite::Connection;
use crate::global::global::PATHS;

const DATABASE_NAME: &str = "database.db";
fn get_database_path() -> PathBuf {
    let path = &PATHS.get().unwrap().data;

    path.join(DATABASE_NAME)
}

pub fn init() -> Connection {
    let path = get_database_path();

    Connection::open(path).unwrap()
}