use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupKeys {
    SetupGlobalToggle,
    SetupGlobalVolume,
    SetupKeyboardToggle,
    SetupKeyboardVolume,
    SetupMouseToggle,
    SetupMouseVolume,
}

pub struct Setup {
    pub setup_global_toggle: Arc<AtomicBool>,
    pub setup_global_volume: Arc<Mutex<f32>>,
    pub setup_keyboard_toggle: Arc<AtomicBool>,
    pub setup_keyboard_volume: Arc<Mutex<f32>>,
    pub setup_mouse_toggle: Arc<AtomicBool>,
    pub setup_mouse_volume: Arc<Mutex<f32>>,
}

#[derive(serde::Serialize)]
pub struct SetupDTO {
    pub setup_global_toggle: bool,
    pub setup_global_volume: f32,
    pub setup_keyboard_toggle: bool,
    pub setup_keyboard_volume: f32,
    pub setup_mouse_toggle: bool,
    pub setup_mouse_volume: f32,
}

impl From<&Setup> for SetupDTO {
    fn from(setup: &Setup) -> Self {
        Self {
            setup_global_toggle: setup.setup_global_toggle.load(Ordering::Relaxed),
            setup_global_volume: *setup.setup_global_volume.lock().unwrap(),
            setup_keyboard_toggle: setup.setup_keyboard_toggle.load(Ordering::Relaxed),
            setup_keyboard_volume: *setup.setup_keyboard_volume.lock().unwrap(),
            setup_mouse_toggle: setup.setup_mouse_toggle.load(Ordering::Relaxed),
            setup_mouse_volume: *setup.setup_mouse_volume.lock().unwrap(),
        }
    }
}