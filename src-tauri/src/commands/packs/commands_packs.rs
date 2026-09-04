use std::fs;
use directories::ProjectDirs;
use opener;
use crate::global::global::{PACKS, PATHS};
use crate::types::packs::type_packs::Pack;


pub fn command_open_packs() {
    let path = &PATHS.get().unwrap().packs;

    opener::open(path).unwrap();
}

pub fn command_display_pack() -> Vec<Pack> {
    let list = PACKS.get().unwrap().lock().unwrap();

    list.clone()
}