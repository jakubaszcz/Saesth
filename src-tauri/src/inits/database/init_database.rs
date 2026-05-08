use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;
use rusqlite::Connection;

const DATABASE_NAME: &str = "database.db";

const DATABASE_QUALIFIER : &str = "com";
const DATABASE_ORGANISATION : &str = "saesth";
const DATABASE_APPLICATION : &str = "saesth";
fn get_database_path() -> PathBuf {
    let directory = ProjectDirs::from(DATABASE_QUALIFIER, DATABASE_ORGANISATION, DATABASE_APPLICATION).unwrap();

    let local = directory.data_dir();
    fs::create_dir_all(local).unwrap();


    local.join(DATABASE_NAME)
}

pub fn init() -> Connection {
    let path = get_database_path();

    Connection::open(path).unwrap()
}