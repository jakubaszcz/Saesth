use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Pack {
    name: String,
    pub(crate) id: String,
    description: String,
    pub(crate) icon: String,
}

#[derive(Debug)]
pub struct SelectedPack {
    pub id: String,
    pub root: PathBuf,
    pub sound: PathBuf
}