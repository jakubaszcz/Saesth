use std::sync::atomic::Ordering;
use tauri::async_runtime::handle;
use crate::functions::sounds::stop::fade::function_sound_fade::function_sound_fade;
use crate::types::sounds::type_sounds::Sound;

pub fn function_sound_stop(sound: &mut Sound) {
    if !sound.player.is_some() {
        return;
    }

    sound.play.store(false, Ordering::Relaxed);

    let Some(player) = sound.player.take() else {
        return;
    };
    let Some(handle) = sound.handle.take() else {
        return;
    };

    let fade_volume = sound.fade_volume.clone();
    let user_volume = sound.volume.clone();
    let drift_volume = sound.drift_volume.clone();

    let player_copy = player.clone();
    let play_flag = sound.play.clone();

    function_sound_fade(
        play_flag,
        player_copy,
        user_volume,
        fade_volume,
        drift_volume,
        handle,
    );
}