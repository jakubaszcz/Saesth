use std::fs;
use directories::ProjectDirs;

const DATABASE_QUALIFIER : &str = "com";
const DATABASE_ORGANISATION : &str = "saesth";
const DATABASE_APPLICATION : &str = "saesth";
pub fn init() {
    let directory = ProjectDirs::from(DATABASE_QUALIFIER, DATABASE_ORGANISATION, DATABASE_APPLICATION).unwrap();

    let local = directory.data_dir();

    fs::create_dir_all(local).unwrap();

    let pack_dir = local.join("pack");
    fs::create_dir_all(&pack_dir).unwrap();
}