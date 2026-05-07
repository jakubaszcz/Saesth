use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use crate::types::setup::type_setup::Setup;

pub fn init() -> Setup {
    Setup {
        setup_global_toggle: Arc::new(AtomicBool::new(true)),
        setup_global_volume: Arc::new(Mutex::new(0.5)),
        setup_keyboard_toggle: Arc::new(AtomicBool::new(true)),
        setup_keyboard_volume: Arc::new(Mutex::new(0.5)),
        setup_mouse_toggle: Arc::new(AtomicBool::new(true)),
        setup_mouse_volume: Arc::new(Mutex::new(0.5))
    }
}