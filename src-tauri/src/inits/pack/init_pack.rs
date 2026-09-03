use std::fs;
use std::fs::File;
use std::iter::zip;
use std::path::PathBuf;
use directories::ProjectDirs;
use crate::commands::packs::commands_packs::command_open_packs;
use crate::types::packs::type_packs::Pack;
use zip::ZipArchive;

const DATABASE_QUALIFIER : &str = "com";
const DATABASE_ORGANISATION : &str = "saesth";
const DATABASE_APPLICATION : &str = "saesth";
pub fn init() -> Vec<Pack>{
    let directory = ProjectDirs::from(DATABASE_QUALIFIER, DATABASE_ORGANISATION, DATABASE_APPLICATION).unwrap();
    let local = directory.data_dir();

    fs::create_dir_all(local).unwrap();

    let pack_dir = local.join("pack");
    fs::create_dir_all(&pack_dir).unwrap();


    let mut packs = Vec::new();

    for entry in fs::read_dir(&pack_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|ext| ext.to_str()) == Some("zip") {
            packs.push(read_pack(path));
        }
    }
    packs
}

fn read_pack(path: PathBuf) -> Pack {
    let file = File::open(&path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();
    let manifest = archive.by_name("manifest.json").unwrap();

    serde_json::from_reader(manifest).unwrap()
}