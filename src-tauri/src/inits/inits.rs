use std::sync::Mutex;
use crate::global::global::{DATABASE, PACKS, PATHS, SETTINGS, SETUP, SOUNDS};
use crate::inits;

pub fn inits() {
    PATHS.get_or_init(inits::appdata::init_appdata::init);
    DATABASE.set(Mutex::new(inits::database::init_database::init())).unwrap();
    SETUP.get_or_init(|| Mutex::new(inits::setup::init_setup::init()));
    SETTINGS.get_or_init(|| Mutex::new(inits::settings::init_settings::init()));
    PACKS.get_or_init(|| Mutex::new(inits::pack::init_pack::init()));

    inits::discord_rich_presence::init_discord_rich_presence::init();
}