use std::fs;
use std::sync::atomic::Ordering;
use rdev::Key::PrintScreen;
use rusqlite::fallible_iterator::FallibleIterator;
use tauri::AppHandle;
use crate::commands::manifest::commands_manifest::commands_manifest_change_volume;
use crate::database::sounds::database_sounds::{database_set_sound_effect_active};
use crate::functions;
use crate::functions::sounds::utils::function_sound_util_volume::function_sound_util_volume;
use crate::global::global::{PATHS, SOUNDS};
use crate::types::sounds::type_sounds::SoundDTO;

pub fn commands_sounds_fetch_sounds() -> Vec<SoundDTO> {
    let list = SOUNDS.get().unwrap().lock().unwrap();


    let list = list.iter()
        .map(|sound| SoundDTO::from(sound))
        .collect();

    list
}

pub fn commands_sounds_toggle_sound(sound_id: String) -> bool {
    let list = SOUNDS.get().unwrap().lock().unwrap();

    list.iter()
        .find(|s| s.sound_id == sound_id)
        .map(|s| {
            let new_val = !s.play.load(Ordering::Relaxed);
            s.play.store(new_val, Ordering::Relaxed);
            new_val
        })
        .unwrap_or(false)
}

pub fn commands_sounds_volume_sound(sound_id: String, volume: f32) -> Result<f32, String> {
    let list = SOUNDS.get().unwrap().lock().unwrap();

    list.iter()
        .find(|s| s.sound_id == sound_id)
        .map(|s| {
            commands_manifest_change_volume(&sound_id, volume)?;
            *s.volume.lock().unwrap() = volume;

            if let Some(player) = &s.player {
                function_sound_util_volume(
                    player,
                    &s.volume,
                    &s.fade_volume,
                    &s.drift_volume,
                );
            }

            Ok(volume)
        })
        .unwrap_or_else(|| Err(format!("Sound not found: {}", sound_id)))
}

pub fn commands_sounds_toggle_sound_effect(sound_id: String, effect_id: String) -> bool {
    let list = SOUNDS.get().unwrap().lock().unwrap();

    list.iter()
        .find(|s| s.sound_id == sound_id)
        .and_then(|s| {
            s.effects.iter()
                .find(|e| e.effect_id == effect_id)
                .map(|e| {
                    let new_val = !e.active.load(Ordering::Relaxed);
                    e.active.store(new_val, Ordering::Relaxed);

                    {
                        database_set_sound_effect_active(sound_id.as_str(), effect_id.as_str(), new_val)
                    }

                    new_val
                })
        })
        .unwrap_or(false)
}
