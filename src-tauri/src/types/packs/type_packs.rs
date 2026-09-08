use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::types::manifest::type_manifest::ManifestSetup;

#[derive(Clone, Serialize, Deserialize)]
pub struct Pack {
    name: String,
    pub(crate) id: String,
    description: String,
    pub(crate) icon: String,
}

#[derive(Debug, Clone)]
pub struct SelectedPack {
    pub id: String,
    pub root: PathBuf,
    pub sound: PathBuf,
    pub setup: ManifestSetup
}
