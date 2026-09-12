use std::{fs, io::Write, path::Path};
use crate::global::global::{MANIFEST, PATHS};
use crate::types::manifest::type_manifest::ManifestData;

fn write(path: &Path, manifest: &ManifestData) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(manifest).map_err(|e| e.to_string())?;
    let mut file = tempfile::NamedTempFile::new_in(path.parent().ok_or("Missing parent directory")?)
        .map_err(|e| e.to_string())?;
    file.write_all(&bytes).map_err(|e| e.to_string())?;
    file.as_file().sync_all().map_err(|e| e.to_string())?;
    file.persist(path).map_err(|e| e.to_string())?;
    Ok(())
}

fn load(path: &Path) -> Result<ManifestData, String> {
    match fs::read(path) {
        Ok(bytes) => serde_json::from_slice(&bytes).map_err(|e| format!("Invalid manifest {}: {e}", path.display())),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let manifest = ManifestData::default();
            write(path, &manifest)?;
            Ok(manifest)
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn init() -> Result<ManifestData, String> {
    load(&PATHS.get().unwrap().data.join("manifest.json"))
}

pub fn update(change: impl FnOnce(&mut ManifestData)) -> Result<(), String> {
    let mut current = MANIFEST.get().ok_or("Manifest not initialized")?.lock().map_err(|e| e.to_string())?;
    let mut next = current.clone();
    change(&mut next);
    write(&PATHS.get().unwrap().data.join("manifest.json"), &next)?;
    *current = next;
    Ok(())
}

pub fn setting_active(id: &str) -> bool {
    MANIFEST.get().unwrap().lock().unwrap().settings.get(id).copied().unwrap_or(false)
}