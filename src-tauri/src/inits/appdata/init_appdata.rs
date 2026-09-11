use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use directories::ProjectDirs;
use crate::types::appdata::type_appdata::Appdata;

const QUALIFIER: &str = "com";
const ORGANISATION: &str = "saesth";
const APPLICATION: &str = "saesth";

pub fn init() -> Appdata {
    let directories = ProjectDirs::from(
        QUALIFIER,
        ORGANISATION,
        APPLICATION,
    ).unwrap();

    let data = directories.data_dir().to_path_buf();
    let cache = directories.cache_dir().to_path_buf();

    let packs = data.join("packs");
    let packs_cache = cache.join("packs");

    fs::create_dir_all(&packs).unwrap();
    fs::create_dir_all(&packs_cache).unwrap();

    Appdata {
        data,
        cache,
        packs,
        packs_cache
    }
}