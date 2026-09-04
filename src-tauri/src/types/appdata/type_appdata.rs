use std::path::PathBuf;

pub struct Appdata {
    pub data: PathBuf,
    pub cache: PathBuf,
    pub packs: PathBuf,
    pub packs_cache: PathBuf,
}