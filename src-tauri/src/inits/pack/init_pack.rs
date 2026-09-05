use std::{fs, io};
use std::fs::File;
use std::iter::zip;
use std::path::{Path, PathBuf};
use directories::ProjectDirs;
use crate::commands::packs::commands_packs::command_open_packs;
use crate::types::packs::type_packs::Pack;
use zip::ZipArchive;
use crate::global::global::PATHS;

pub fn init() -> Vec<Pack> {
    let path = &PATHS.get().unwrap().packs;
    let cache = &PATHS.get().unwrap().packs_cache;

    let mut packs = Vec::new();

    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let zip = entry.path();

        if zip.extension().and_then(|ext| ext.to_str()) == Some("zip") {
            packs.push(read_pack(zip, cache));
        }
    }
    packs
}

fn read_pack(path: PathBuf, cache: &Path) -> Pack {
    let file = File::open(&path).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    let mut config: Pack = {
        let manifest = archive.by_name("manifest.json").unwrap();
        serde_json::from_reader(manifest).unwrap()
    };

    let pack_cache = cache.join(&config.id);
    let pack_sound_cache = pack_cache.join("sounds");
    let pack_icon_cache = pack_cache.join("icon.png");

    fs::create_dir_all(&pack_sound_cache).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let Some(enclosed_path) = file.enclosed_name() else {
            continue;
        };

        let Ok(relative_path) = enclosed_path.strip_prefix("sounds") else {
            continue;
        };

        if relative_path.as_os_str().is_empty() {
            continue;
        }

        let output_path = pack_sound_cache.join(relative_path);

        if file.is_dir() {
            fs::create_dir_all(&output_path).unwrap();
            continue;
        }

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        let mut output = File::create(&output_path).unwrap();
        io::copy(&mut file, &mut output).unwrap();
    }

    {
        let mut icon = archive.by_name("icon.png").unwrap();
        let mut output = File::create(&pack_icon_cache).unwrap();

        io::copy(&mut icon, &mut output).unwrap();
    }

    config.icon = pack_icon_cache.to_string_lossy().to_string();

    config
}