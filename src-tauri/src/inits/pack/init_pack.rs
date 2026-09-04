use std::{fs, io};
use std::fs::File;
use std::iter::zip;
use std::path::PathBuf;
use directories::ProjectDirs;
use crate::commands::packs::commands_packs::command_open_packs;
use crate::types::packs::type_packs::Pack;
use zip::ZipArchive;
use crate::global::global::PATHS;

pub fn init() -> Vec<Pack> {


    let path = &PATHS.get().unwrap().packs;

    println!("{}", path.display());

    let mut packs = Vec::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let zip = entry.path();

        if zip.extension().and_then(|ext| ext.to_str()) == Some("zip") {
            println!("{}", zip.display());
            packs.push(read_pack(zip, path));
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

    println!("{} icon path", icon_path.display());

    config
}