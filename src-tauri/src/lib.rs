use crate::types::sounds::type_sounds::{Sound, SoundDTO};
use rodio::Source;
use std::sync::{Mutex, OnceLock};
use tauri::{Emitter, Manager};
use crate::database::settings::database_settings::database_settings_get_active_setting;
use crate::functions::functions::functions;
use crate::global::global::PREFIX_FOR_SETTING;
use crate::types::settings::type_settings::{SettingDTO, SettingKeys};
use crate::types::setup::type_setup::{SetupDTO};
use crate::utils::prefix::util_prefix::util_prefix_add_prefix;

mod database;
mod inits;
mod types;
mod global;
mod utils;

mod functions;

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

#[tauri::command]
fn fetch_settings() -> Vec<SettingDTO> {
    commands::settings::commands_settings::commands_settings_fetch_settings()
}

#[tauri::command]
fn toggle_setting(setting_id: String) -> bool {
    commands::settings::commands_settings::commands_settings_toggle_setting(setting_id)
}

#[tauri::command]
fn fetch_setup() -> Vec<SetupDTO> {
    commands::setup::commands_setup::commands_setup_fetch_setup()
}

#[tauri::command]
fn toggle_setup(setup_id: String) -> bool {
    commands::setup::commands_setup::commands_setup_toggle_setup(setup_id)
}

#[tauri::command]
fn volume_setup(setup_id: String, value: f32) -> f32 {
    commands::setup::commands_setup::commands_setup_volume_setup(setup_id, value)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    inits::inits::inits();

    functions();

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
            if database_settings_get_active_setting(format!("{}_{}", PREFIX_FOR_SETTING, SettingKeys::MinimizeToTray.to_key()).as_str()) {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    window.hide().unwrap();
                }
            }
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if database_settings_get_active_setting(format!("{}_{}", PREFIX_FOR_SETTING, SettingKeys::SingleInstance.to_key()).as_str()) {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
        }))
        .invoke_handler(tauri::generate_handler![
            fetch_sounds,
            toggle_sound,
            toggle_sound_effect,
            volume_sound,
            fetch_setup,
            toggle_setup,
            volume_setup,
            fetch_settings,
            toggle_setting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}