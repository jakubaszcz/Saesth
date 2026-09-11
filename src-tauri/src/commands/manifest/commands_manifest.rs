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

/// Persist one setup control while preserving the other manifest fields.
/// The caller holds SETUP so concurrent control updates stay ordered.
pub fn commands_manifest_change_setup(
    setup_id: &str,
    volume: Option<f32>,
    active: Option<bool>,
) -> Result<(), String> {
    if let Some(value) = volume {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err("Volume must be between 0 and 1".into());
        }
    }
    let id = setup_id.strip_prefix(&format!("{}_", crate::global::global::PREFIX_FOR_SETUP))
        .filter(|id| matches!(*id, "global" | "keyboard" | "mouse"))
        .ok_or("Invalid setup ID")?;
    let mut pack = PACK.get().ok_or("No active pack")?.lock().map_err(|e| e.to_string())?;
    if pack.id.is_empty() { return Err("No active pack".into()); }
    let path = pack.root.join("manifest.json");
    let mut manifest: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let control = manifest.get_mut("setup").and_then(|setup| setup.get_mut(id))
        .and_then(Value::as_object_mut).ok_or("Setup not found in manifest")?;
    if let Some(value) = volume { control.insert("volume".into(), serde_json::json!(value)); }
    if let Some(value) = active { control.insert("active".into(), serde_json::json!(value)); }
    let setup = serde_json::from_value(manifest["setup"].clone()).map_err(|e| e.to_string())?;
    fs::write(&path, serde_json::to_vec_pretty(&manifest).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    pack.setup = setup;
    Ok(())
}
