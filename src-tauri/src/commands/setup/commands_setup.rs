use std::sync::atomic::Ordering;
use crate::commands::manifest::commands_manifest::commands_manifest_change_setup;
use crate::global::global::SETUP;
use crate::types::setup::type_setup::SetupDTO;

pub fn commands_setup_fetch_setup() -> Vec<SetupDTO> {
    SETUP.get().unwrap().lock().unwrap().iter().map(SetupDTO::from).collect()
}

pub fn commands_setup_volume_setup(setup_id: String, volume: f32) -> Result<f32, String> {
    let setups = SETUP.get().ok_or("Setup not initialized")?.lock().map_err(|e| e.to_string())?;
    let setup = setups.iter().find(|setup| setup.setup_id == setup_id).ok_or("Unknown setup ID")?;
    let mut current_volume = setup.volume.lock().map_err(|e| e.to_string())?;
    commands_manifest_change_setup(&setup_id, Some(volume), None)?;
    *current_volume = volume;
    Ok(volume)
}

pub fn commands_setup_toggle_setup(setup_id: String) -> Result<bool, String> {
    let setups = SETUP.get().ok_or("Setup not initialized")?.lock().map_err(|e| e.to_string())?;
    let setup = setups.iter().find(|setup| setup.setup_id == setup_id).ok_or("Unknown setup ID")?;
    let active = !setup.toggle.load(Ordering::Relaxed);
    commands_manifest_change_setup(&setup_id, None, Some(active))?;
    setup.toggle.store(active, Ordering::Relaxed);
    Ok(active)
}
