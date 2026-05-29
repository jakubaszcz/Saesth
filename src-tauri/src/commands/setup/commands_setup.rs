use std::sync::atomic::Ordering;
use crate::database::setup::database_setup::{database_set_setup_toggle, database_set_setup_volume};
use crate::global::global::SETUP;
use crate::types::setup::type_setup::{SetupDTO};

pub fn commands_setup_fetch_setup() -> Vec<SetupDTO> {
    let setup = SETUP.get().unwrap().lock().unwrap();

    setup.iter()
        .map(|setup| SetupDTO::from(setup))
        .collect()
}


pub fn commands_setup_volume_setup(setup_id: String, volume: f32) -> f32{
    let setup = SETUP.get().unwrap().lock().unwrap();

    setup.iter()
        .find(|setup| setup.setup_id == setup_id)
        .map(|setup| {
            *setup.volume.lock().unwrap() = volume;

            {
                database_set_setup_volume(setup_id.as_str(), volume);
            }

            volume
        })
        .unwrap_or(0.5)
}

pub fn commands_setup_toggle_setup(setup_id: String) -> bool {
    let setup = SETUP.get().unwrap().lock().unwrap();

    setup.iter()
        .find(|setup| setup.setup_id == setup_id)
        .map(|setup| {
            let new_val = !setup.toggle.load(Ordering::Relaxed);
            setup.toggle.store(new_val, Ordering::Relaxed);

            {
                database_set_setup_toggle(setup_id.as_str(), new_val);
            }

            new_val
        }).unwrap_or(false)
}