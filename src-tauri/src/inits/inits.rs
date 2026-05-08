use std::sync::Mutex;
use crate::global::global;
use crate::global::global::{SETTINGS, SOUNDS};
use crate::inits;

pub fn inits() {
    SOUNDS.get_or_init(|| Mutex::new(inits::sounds::init_sound::init()));
    SETTINGS.get_or_init(|| Mutex::new(inits::settings::init_settings::init()));
}