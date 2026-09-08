use std::fs;
use serde_json::Value;
use crate::global::global::{PACK, PREFIX_FOR_SOUND};

pub fn commands_manifest_change_volume(sound_id: &str, value: f32) -> Result<(), String> {
    let pack = PACK.get().ok_or("No active pack")?.lock().map_err(|e| e.to_string())?;
    if pack.id.is_empty() {
        return Err("No active pack".into());
    }
    let manifest_path = pack.root.join("manifest.json");
    let contents = fs::read(&manifest_path).map_err(|e| e.to_string())?;
    let mut manifest: Value = serde_json::from_slice(&contents).map_err(|e| e.to_string())?;

    change_volume(&mut manifest, sound_id, value)?;

    let contents = serde_json::to_vec_pretty(&manifest).map_err(|e| e.to_string())?;
    fs::write(&manifest_path, contents).map_err(|e| e.to_string())
}

fn change_volume(manifest: &mut Value, sound_id: &str, value: f32) -> Result<(), String> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err("Volume must be between 0 and 1".into());
    }
    let id = sound_id.strip_prefix(&format!("{}_", PREFIX_FOR_SOUND))
        .ok_or("Invalid sound ID")?;
    let sound = manifest["sounds"].as_array_mut()
        .and_then(|sounds| sounds.iter_mut().find(|sound| sound["id"].as_str() == Some(id)))
        .ok_or_else(|| format!("Sound not found in manifest: {}", id))?;
    sound["volume"] = serde_json::json!(value);
    Ok(())
}
