use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use rodio::{DeviceSinkBuilder, Player, Source};
use crate::functions::sounds::drift::function_sound_drift::function_sound_drift;
use crate::functions::sounds::play::fade::function_sound_fade::function_sound_fade;
use crate::types::sounds::type_sounds::Sound;
use crate::utils::prefix::util_prefix::util_prefix_remove_prefix;

const PATH: &str = "./sounds";

pub fn function_sound_play(sound: &mut Sound) {
    if sound.player.is_some() {
        return;
    }

    let path = format!(
        "{}/{}/{}.mp3",
        PATH,
        util_prefix_remove_prefix(&sound.sound_id).as_str(),
        "default");

    let handle = DeviceSinkBuilder::open_default_sink().unwrap();

    let player = Arc::new(
        Mutex::new(
            Player::connect_new(&handle.mixer())
        )
    );

    let file = std::fs::File::open(path).unwrap();

    let source = rodio::Decoder::new(file).unwrap().repeat_infinite();

    player.lock().unwrap().append(source);
    player.lock().unwrap().set_volume(0.0);
    player.lock().unwrap().play();

    let fade_volume = sound.fade_volume.clone();
    let user_volume = sound.volume.clone();
    let drift_volume = sound.drift_volume.clone();

    let clone_player = player.clone();
    let play_flag = sound.play.clone();
    
    // REMOVED: *fade_volume.lock().unwrap() = 0.0;
    // We let it start from whatever it is (e.g. if it was already partially faded in or out)

    function_sound_fade(
        play_flag,
        clone_player,
        user_volume,
        fade_volume,
        drift_volume,
    );

    sound.player = Some(player);
    sound.handle = Some(handle);

    sound.play.store(true, Ordering::Relaxed);

    function_sound_drift(sound);

    /*    song_drift(sound);
        handle_effects(sound);
    */
    }