use std::sync::Mutex;
use crate::global::global::{MANIFEST, PACKS, PATHS, SETTINGS, SETUP, SOUNDS};
use crate::inits;

pub fn inits() {
    SOUNDS.get_or_init(|| Mutex::new(Vec::new()));
    PATHS.get_or_init(inits::appdata::init_appdata::init);
    MANIFEST.set(Mutex::new(inits::manifest::init_manifest::init().expect("Unable to load application manifest"))).unwrap();
    SETUP.get_or_init(|| Mutex::new(inits::setup::init_setup::init()));
    SETTINGS.get_or_init(|| Mutex::new(inits::settings::init_settings::init()));
    PACKS.get_or_init(|| Mutex::new(inits::pack::init_pack::init()));

    inits::discord_rich_presence::init_discord_rich_presence::init();
}