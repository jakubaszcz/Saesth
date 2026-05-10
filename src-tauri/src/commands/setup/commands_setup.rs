use std::sync::atomic::Ordering;
use crate::global::global::SETUP;
use crate::types::setup::type_setup::{SetupDTO, SetupKeys};

pub fn commands_setup_fetch_setup() -> SetupDTO {
    let setup = SETUP.get().unwrap().lock().unwrap();

    SetupDTO::from(&*setup)
}


pub fn commands_setup_volume_setup(key: SetupKeys, value: f32) {
    let setup = SETUP.get().unwrap().lock().unwrap();

    match key {
        SetupKeys::SetupGlobalVolume => {
            *setup.setup_global_volume.lock().unwrap() = value;
        },
        SetupKeys::SetupKeyboardVolume => {
            *setup.setup_keyboard_volume.lock().unwrap() = value;
        },
        SetupKeys::SetupMouseVolume => {
            *setup.setup_mouse_volume.lock().unwrap() = value;
        },
        SetupKeys::SetupGlobalToggle | SetupKeys::SetupKeyboardToggle | SetupKeys::SetupMouseToggle => todo!()
    }
}

pub fn commands_setup_toggle_setup(key: SetupKeys) {
    let setup = SETUP.get().unwrap().lock().unwrap();

    match key {
        SetupKeys::SetupGlobalToggle => {
            setup.setup_global_toggle.store(!setup.setup_global_toggle.load(Ordering::Relaxed), Ordering::Relaxed);
        },
        SetupKeys::SetupKeyboardToggle => {
            setup.setup_keyboard_toggle.store(!setup.setup_keyboard_toggle.load(Ordering::Relaxed), Ordering::Relaxed);
        },
        SetupKeys::SetupMouseToggle => {
            setup.setup_mouse_toggle.store(!setup.setup_mouse_toggle.load(Ordering::Relaxed), Ordering::Relaxed);
        },
        SetupKeys::SetupGlobalVolume | SetupKeys::SetupKeyboardVolume | SetupKeys::SetupMouseVolume => todo!()
    }
}