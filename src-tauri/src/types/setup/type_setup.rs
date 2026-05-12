use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

/*#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupKeys {
    SetupGlobalToggle,
    SetupGlobalVolume,
    SetupKeyboardToggle,
    SetupKeyboardVolume,
    SetupMouseToggle,
    SetupMouseVolume,
}*/

pub struct Setup {
    pub setup_id: String,
    pub toggle: Arc<AtomicBool>,
    pub volume: Arc<Mutex<f32>>,
}

#[derive(serde::Serialize)]
pub struct SetupDTO {
    pub setup_id: String,
    pub toggle: bool,
    pub volume: f32,
}

impl From<&Setup> for SetupDTO {
    fn from(setup: &Setup) -> Self {
        Self {
            setup_id: setup.setup_id.clone(),
            toggle: setup.toggle.load(Ordering::Relaxed),
            volume: *setup.volume.lock().unwrap(),
        }
    }
}