use std::sync::Mutex;
use crate::global::global::{DATABASE, SETTINGS, SOUNDS};
use crate::inits;

pub fn inits() {
    DATABASE.set(Mutex::new(inits::database::init_database::init())).unwrap();
    SOUNDS.get_or_init(|| Mutex::new(inits::sounds::init_sound::init()));
    SETTINGS.get_or_init(|| Mutex::new(inits::settings::init_settings::init()));
}