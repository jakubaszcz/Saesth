use std::fs::File;
use std::sync::Mutex;
use opener;
use crate::inits::manifest::init_manifest::update;
use crate::global::global::{PACK, PACKS, PATHS, SETUP, SOUNDS};
use crate::inits::pack::init_pack::init_pack_sound;
use crate::types::manifest::type_manifest::Manifest;
use crate::types::packs::type_packs::{Pack, SelectedPack};


pub fn command_open_packs() {
    let path = &PATHS.get().unwrap().packs;

    opener::open(path).unwrap();
}

pub fn command_display_pack() -> Vec<Pack> {
    let list = PACKS.get().unwrap().lock().unwrap();

    list.clone()
}

pub fn command_select_pack(id: String) -> Result<(), String> {
    if id.is_empty() || std::path::Path::new(&id).components().count() != 1 || !matches!(std::path::Path::new(&id).components().next(), Some(std::path::Component::Normal(_))) { return Err("Invalid pack ID".into()); }
    let path = &PATHS.get().unwrap().packs_cache;

    let pack = if path.clone().join(id.clone()).exists() {
        path.join(id.clone())
    } else {
        return Err("Pack not found".into());
    };

    let selected_pack = path.join(id.clone());
    let manifest_path = selected_pack.join("manifest.json");
    drop(selected_pack);

    let file = match File::open(manifest_path) {
        Ok(file) => file,
        Err(e) => return Err(e.to_string()),
    };

    let manifest: Manifest = match serde_json::from_reader(file) {
        Ok(m) => m,
        Err(e) => return Err(e.to_string()),
    };

    let sounds = pack.join("sounds");

    if !sounds.is_dir() { return Err("Pack sounds directory missing".into()); }
    update(|manifest| manifest.pack = id.clone())?;

    let selected_pack = SelectedPack {
        id,
        root: pack,
        sound: sounds,
        setup: manifest.setup
    };

    {
        let mut current_pack = PACK
            .get_or_init(|| Mutex::new(selected_pack.clone()))
            .lock()
            .unwrap();
        *current_pack = selected_pack;
    }

    let new_setup = crate::inits::setup::init_setup::init();
    *SETUP.get_or_init(|| Mutex::new(Vec::new())).lock().unwrap() = new_setup;

    let new_sounds = init_pack_sound();
    *SOUNDS.get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap() = new_sounds;
    Ok(())
}

pub fn command_has_active_pack() -> bool {
    command_get_selected_pack().is_some()
}

pub fn command_save_pack(id: String) -> Result<(), String> {
    command_select_pack(id)
}

pub fn command_get_selected_pack() -> Option<String> {
    let pack = PACK.get()?.lock().unwrap();
    if pack.id.is_empty() { None } else { Some(pack.id.clone()) }
}

pub fn command_deselect_pack() -> Result<(), String> {
    update(|manifest| manifest.pack.clear())?;
    if let Some(sounds) = SOUNDS.get() {
        let mut sounds = sounds.lock().unwrap();
        for sound in sounds.iter() {
            sound.play.store(false, std::sync::atomic::Ordering::Relaxed);
            if let Some(player) = &sound.player {
                player.lock().unwrap().stop();
            }
        }
        sounds.clear();
    }
    if let Some(setup) = SETUP.get() {
        setup.lock().unwrap().clear();
    }
    if let Some(pack) = PACK.get() {
        let mut pack = pack.lock().unwrap();
        pack.id.clear();
        pack.root.clear();
        pack.sound.clear();
    }
    Ok(())
}
