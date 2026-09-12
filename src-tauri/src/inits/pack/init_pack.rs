use std::{fs, io};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use crate::commands::packs::commands_packs::{command_select_pack};
use crate::types::packs::type_packs::Pack;
use zip::ZipArchive;
use crate::global::global::{PACK, PATHS};
use crate::types::manifest::type_manifest::{Manifest, ManifestSounds};
use crate::types::sounds::type_sounds::{Effect, Sound};


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
    let selected = crate::global::global::MANIFEST.get().unwrap().lock().unwrap().pack.clone();
    if !selected.is_empty() {
        if let Err(error) = command_select_pack(selected) {
            eprintln!("Unable to restore selected pack: {error}");
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
    let pack_icon_cache = pack_cache.join("icon.png");

    fs::create_dir_all(&pack_cache).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();

        let Some(enclosed_path) = file.enclosed_name() else {
            continue;
        };

        let output_path = pack_cache.join(enclosed_path);

        if file.is_dir() {
            fs::create_dir_all(&output_path).unwrap();
            continue;
        }

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        if output_path == pack_cache.join("manifest.json") && output_path.is_file() {
            continue;
        }

        let mut output = File::create(&output_path).unwrap();
        io::copy(&mut file, &mut output).unwrap();
    }

    config.icon = pack_icon_cache.to_string_lossy().to_string();

    config
}

fn make_stream(id: &str, effects: Vec<Effect>, config: &ManifestSounds) -> Sound {

    let sound_id = id.to_string();

    Sound {
        sound_id: sound_id.clone(),
        handle: None,
        player: None,
        play: Arc::new(AtomicBool::new(false)),
        volume: Arc::new(Mutex::new(config.volume)),
        fade_volume: Arc::new(Mutex::new(0.0)),
        drift_volume: Arc::new(Mutex::new(1.0)),
        effects
    }

}

fn make_effect(id: &str) -> Effect {

    let effect_id = id.to_string();

    Effect {
        effect_id: effect_id.clone(),
        active: Arc::new(AtomicBool::new(false)),
    }
}

pub fn init_pack_sound() -> Vec<Sound> {
    let Some(pack) = PACK.get() else {
        return Vec::new();
    };

    let selected_pack = pack.lock().unwrap();
    if selected_pack.id.is_empty() {
        return Vec::new();
    }
    let manifest_path = selected_pack.root.join("manifest.json");

    drop(selected_pack);

    let file = match File::open(manifest_path) {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };

    let manifest: Manifest = match serde_json::from_reader(file) {
        Ok(m) => m,
        Err(_) => return Vec::new(),
    };

    manifest.sounds
        .iter()
        .map(|sound| {
            let effects = sound.effects
                .iter()
                .map(|effect| make_effect(effect))
                .collect();

            make_stream(&sound.id, effects, &sound)
        })
        .collect()
}