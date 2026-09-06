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

    PACK.get_or_init(|| SelectedPack {
        id,
        root: pack,
        sound: sounds.exists().then_some(sounds).expect("REASON"),
    });

    SOUNDS.get_or_init(|| Mutex::new(init_pack_sound()));
}

pub fn command_save_pack(id: String) {
    println!("save pack");

    let path = &PATHS.get().unwrap().packs_cache;

    let pack = if path.clone().join(id.clone()).exists() {
        path.join(id.clone())
    } else {
        return;
    };

    database_pack_set_active_pack(id.clone());

    let sounds = pack.join("sounds");

    PACK.get_or_init(|| SelectedPack {
        id,
        root: pack,
        sound: sounds.exists().then_some(sounds).expect("REASON"),
    });

    SOUNDS.get_or_init(|| Mutex::new(init_pack_sound()));

}