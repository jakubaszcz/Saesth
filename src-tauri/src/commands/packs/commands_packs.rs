use std::fs;
use std::path::PathBuf;
use std::ptr::null;
use std::sync::Mutex;
use directories::ProjectDirs;
use opener;
use tauri::{AppHandle, Emitter};
use crate::database::packs::database_packs::database_pack_set_active_pack;
use crate::global::global::{PACK, PACKS, PATHS, SOUNDS};
use crate::inits::pack::init_pack::init_pack_sound;
use crate::types::packs::type_packs::{Pack, SelectedPack};


pub fn command_open_packs() {
    let path = &PATHS.get().unwrap().packs;

    opener::open(path).unwrap();
}

pub fn command_display_pack() -> Vec<Pack> {
    let list = PACKS.get().unwrap().lock().unwrap();

    list.clone()
}

pub fn command_select_pack(id: String) {
    let path = &PATHS.get().unwrap().packs_cache;

    let pack = if path.clone().join(id.clone()).exists() {
        path.join(id.clone())
    } else {
        return;
    };

    let sounds = pack.join("sounds");

    let selected_pack = SelectedPack {
        id,
        root: pack,
        sound: sounds.exists().then_some(sounds).expect("REASON"),
    };

    {
        let mut current_pack = PACK
            .get_or_init(|| Mutex::new(selected_pack.clone()))
            .lock()
            .unwrap();
        *current_pack = selected_pack;
    }

    let new_sounds = init_pack_sound();
    *SOUNDS.get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap() = new_sounds;
}

pub fn command_has_active_pack() -> bool {
    command_get_selected_pack().is_some()
}

pub fn command_save_pack(id: String) {
    let path = &PATHS.get().unwrap().packs_cache;

    if !path.join(&id).exists() {
        return;
    }

    database_pack_set_active_pack(id.clone());

    command_select_pack(id);
}

pub fn command_get_selected_pack() -> Option<String> {
    let pack = PACK.get()?.lock().unwrap();
    if pack.id.is_empty() { None } else { Some(pack.id.clone()) }
}

pub fn command_deselect_pack() {
    database_pack_set_active_pack(String::new());
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
    if let Some(pack) = PACK.get() {
        let mut pack = pack.lock().unwrap();
        pack.id.clear();
        pack.root.clear();
        pack.sound.clear();
    }
}
