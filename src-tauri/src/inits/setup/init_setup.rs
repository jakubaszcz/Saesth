use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use crate::global::global::{PACK, PREFIX_FOR_SETUP};
use crate::types::setup::type_setup::Setup;

pub fn init() -> Vec<Setup> {
    let Some(pack) = PACK.get() else { return Vec::new(); };
    let pack = pack.lock().unwrap();
    if pack.id.is_empty() { return Vec::new(); }
    let config = &pack.setup;
    [
        ("global", config.global.active, config.global.volume),
        ("keyboard", config.keyboard.active, config.keyboard.volume),
        ("mouse", config.mouse.active, config.mouse.volume),
    ].into_iter().map(|(id, active, volume)| Setup {
        setup_id: format!("{}_{}", PREFIX_FOR_SETUP, id),
        toggle: Arc::new(AtomicBool::new(active)),
        volume: Arc::new(Mutex::new(volume)),
    }).collect()
}
