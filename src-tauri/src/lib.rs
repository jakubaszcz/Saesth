use crate::types::sounds::type_sounds::{Sound, SoundDTO};
use rodio::Source;
use std::sync::{Mutex, OnceLock};
use tauri::{Emitter, Manager};
use crate::types::settings::type_settings::Setting;

mod database;
mod sounds;
mod inits;
mod types;
mod global;

mod commands;

#[tauri::command]
fn fetch_sounds() -> Vec<SoundDTO> {
    commands::sounds::commands_sounds::commands_sounds_fetch_sounds()
}

#[tauri::command]
fn toggle_sound(sound_id: String) -> bool {
    commands::sounds::commands_sounds::commands_sounds_toggle_sound(sound_id)
}

#[tauri::command]
fn toggle_sound_effect(sound_id: String, effect_id: String) -> bool {
    commands::sounds::commands_sounds::commands_sounds_toggle_sound_effect(sound_id, effect_id)
}

#[tauri::command]
fn volume_sound(sound_id: String, volume: f32) -> f32 {
    commands::sounds::commands_sounds::commands_sounds_volume_sound(sound_id, volume)
}

/*#[tauri::command]
fn set_settings(id: String, value: String) {
    database::database::set_setting(&*id, &*value);
}

#[tauri::command]
fn get_settings(id: String) -> String {
    database::database::get_setting(id.as_str())
}
#[tauri::command]*/
fn toggle_setup(key: types::setup::type_setup::SetupKeys) {
    sounds::setup::setup::toggle_setup(key)
}

#[tauri::command]
fn volume_setup(key: types::setup::type_setup::SetupKeys, value: f32) {
    sounds::setup::setup::volume_setup(key, value);
}

#[tauri::command]
fn fetch_setup() -> types::setup::type_setup::SetupDTO {
    sounds::setup::setup::fetch_setup()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    inits::inits::inits();

    sounds::setup::setup::setup();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            let handle = app.handle().clone();

            // Tray
            inits::tray::init_tray::init(app);

            // Focus on opening
            window.set_focus().unwrap();

            // Hide the window title bar
            window.set_decorations(false).unwrap();

            // Window position
            {
                let window_size = window.outer_size().unwrap();

                let current_screen = window.current_monitor().unwrap().unwrap();
                let screen_size = current_screen.size();

                let position_x = (screen_size.width - window_size.width) / 2;
                let position_y = (screen_size.height - window_size.height) / 2;

                window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                    x: position_x as i32,
                    y: position_y as i32,
                })).unwrap();
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            /*if database::database::get_setting("close_to_tray") == "true" {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    window.hide().unwrap();
                }
            }*/
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_sounds,
            toggle_sound,
            toggle_sound_effect,
            volume_sound,
            fetch_setup,
            volume_setup,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}