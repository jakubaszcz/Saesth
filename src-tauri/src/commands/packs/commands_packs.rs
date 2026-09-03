use std::fs;
use directories::ProjectDirs;
use opener;
const DATABASE_QUALIFIER : &str = "com";
const DATABASE_ORGANISATION : &str = "saesth";
const DATABASE_APPLICATION : &str = "saesth";
pub fn command_open_packs() {
    let directory = ProjectDirs::from(DATABASE_QUALIFIER, DATABASE_ORGANISATION, DATABASE_APPLICATION).unwrap();
    let local = directory.data_dir();

    fs::create_dir_all(local).unwrap();

    let pack_dir = local.join("pack");

    opener::open(&pack_dir).unwrap();
}