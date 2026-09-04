use std::{fs, io};
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
pub fn init() -> Vec<Pack> {
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
            packs.push(read_pack(path, &pack_dir));
        }
    }
    packs
}

fn read_pack(path: PathBuf, pack_dir: &PathBuf) -> Pack {
    let file = File::open(&path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    let mut config: Pack = {
        let manifest = archive.by_name("manifest.json").unwrap();
        serde_json::from_reader(manifest).unwrap()
    };

    let mut icon = archive.by_name("icon.png").unwrap();

    let icon_file_name = format!("{}.png", path.file_stem().unwrap().to_string_lossy());
    let icon_path = pack_dir.join(icon_file_name);

    let mut output = File::create(&icon_path).unwrap();

    io::copy(&mut icon, &mut output).unwrap();

    config.icon = icon_path.to_string_lossy().to_string();

    config
}