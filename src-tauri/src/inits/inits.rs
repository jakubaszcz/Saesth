use std::sync::Mutex;
use crate::global::global::{DATABASE, SETTINGS, SETUP, SOUNDS};
use crate::inits;

pub fn inits() {
    DATABASE.set(Mutex::new(inits::database::init_database::init())).unwrap();
    SETUP.get_or_init(|| Mutex::new(inits::setup::init_setup::init()));
    SOUNDS.get_or_init(|| Mutex::new(inits::sounds::init_sound::init()));
    SETTINGS.get_or_init(|| Mutex::new(inits::settings::init_settings::init()));

    inits::discord_rich_presence::init_discord_rich_presence::init();
    inits::pack::init_pack::init();
}