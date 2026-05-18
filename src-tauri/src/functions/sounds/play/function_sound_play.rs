use std::sync::{Arc, Mutex};
use std::sync::atomic::Ordering;
use rodio::{DeviceSinkBuilder, Player, Source};
use crate::types::sounds::type_sounds::Sound;
use crate::utils::prefix::util_prefix::util_prefix_remove_prefix;

pub fn function_sound_play(sound: &mut Sound) {
    if sound.player.is_some() {
        return;
    }

    let path = format!(
        "{}/{}.mp3",
        util_prefix_remove_prefix(&sound.sound_id).as_str(),
        "default");

    let handle = DeviceSinkBuilder::open_default_sink().unwrap();

    let player = Arc::new(
        Mutex::new(
            Player::connect_new(&handle.mixer())
        )
    );

    let file = std::fs::File::open(format!("./sounds/{}", path)).unwrap();

    let source = rodio::Decoder::new(file).unwrap().repeat_infinite();

    player.lock().unwrap().append(source);
    player.lock().unwrap().set_volume(0.0);
    player.lock().unwrap().play();

    let fade_volume = sound.fade_volume.clone();
/*    let user_volume = sound.volume.clone();
    let drift_volume = sound.drift_volume.clone();*/

    {
        *fade_volume.lock().unwrap() = 0.0;
    }

    sound.player = Some(player);
    sound.handle = Some(handle);
    sound.play.store(true, Ordering::Relaxed);

}