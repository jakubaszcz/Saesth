use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use rodio::{DeviceSinkBuilder, Player, Source};
use tauri::ipc::RuntimeCapability;
use crate::functions::sounds::drift::function_sound_drift::function_sound_drift;
use crate::functions::sounds::effect::function_sound_effect::function_sound_effect;
use crate::functions::sounds::play::fade::function_sound_fade::function_sound_fade;
use crate::global::global::PACK;
use crate::types::sounds::type_sounds::Sound;

pub fn function_sound_play(sound: &mut Sound) {
    if sound.player.is_some() {
        return;
    }

    let sounds_path = &PACK.get().unwrap();


    let sounds_path = if sounds_path.lock().unwrap().sound.join("sounds").exists() {
        sounds_path.lock().unwrap().sound.join("sounds")
    } else {
        sounds_path.lock().unwrap().sound.join("../sounds")
    };

    let path = sounds_path
        .join(&sound.sound_id)
        .join("default.mp3");

    let handle = DeviceSinkBuilder::open_default_sink().unwrap();

    let player = Arc::new(
        Mutex::new(
            Player::connect_new(&handle.mixer())
        )
    );

    let file = match std::fs::File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open sound file at {:?}: {}", path, e);
            return;
        }
    };

    let source = rodio::Decoder::new(file).unwrap().repeat_infinite();

    player.lock().unwrap().append(source);
    player.lock().unwrap().set_volume(0.0);
    player.lock().unwrap().play();

    let fade_volume = sound.fade_volume.clone();
    let user_volume = sound.volume.clone();
    let drift_volume = sound.drift_volume.clone();

    let clone_player = player.clone();
    let play_flag = sound.play.clone();
    let mixer = handle.mixer().clone();

    function_sound_fade(
        play_flag.clone(),
        clone_player,
        user_volume.clone(),
        fade_volume.clone(),
        drift_volume.clone(),
    );

    sound.player = Some(player);
    sound.handle = Some(handle);

    sound.play.store(true, Ordering::Relaxed);

    function_sound_drift(sound);

    for effect in &sound.effects {
        function_sound_effect(
            effect.clone(),
            play_flag.clone(),
            user_volume.clone(),
            fade_volume.clone(),
            drift_volume.clone(),
            mixer.clone(),
        );
    }

    }