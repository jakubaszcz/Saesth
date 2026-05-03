use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use rdev::{listen, Event, EventType};
use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player};
use crate::sounds::random_sound;
use crate::utils;

enum Type {
    Keybind,
    Mouse,
}

fn play_sound(player: &Arc<Mutex<Player>>) {
    let path = random_sound::random_sound("sounds/setup/keyboard");

    let Ok(file) = std::fs::File::open(&path) else {
        return;
    };

    let Ok(source) = rodio::Decoder::try_from(file) else {
        return;
    };

    let player = player.lock().unwrap();
    player.stop();
    player.append(source);
    player.play();
}

pub fn setup() {

    let handle = DeviceSinkBuilder::open_default_sink()
        .expect("failed to open default audio device");

    let player = Arc::new(Mutex::new(
        Player::connect_new(&handle.mixer())
    ));

    let player_clone = player.clone();

    thread::spawn(move || {
        listen(move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key ) => {
                    println!("{:?}", key);
                    play_sound(&player_clone);
                }
                _ => {}
            }
        }).unwrap();

        drop(handle);
    });
}