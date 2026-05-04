use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use rdev::{listen, Event, EventType};
use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player};
use crate::sounds::random_sound;
use crate::utils;

enum Type {
    Keys,
    Space,
    Delete,
    LMB,
    RMB,
}

fn convert_type(key: &Type) -> Option<String> {
    match key {
        Type::Keys => Some("keys".to_string()),
        Type::Space => Some("space".to_string()),
        Type::Delete => Some("delete".to_string()),
        Type::LMB => Some("left_mouse_button".to_string()),
        Type::RMB => Some("right_mouse_button".to_string()),
        _ => None
    }
}

fn play_sound(key: Type, player: &Arc<Mutex<Player>>) {
    let path = random_sound::random_sound(
        &format!("sounds/setup/{}", convert_type(&key).unwrap())
    );

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

                    match key {
                        rdev::Key::Space => {
                            play_sound(Type::Space, &player_clone);
                            println!("space");
                        }
                        rdev::Key::Delete => {
                            play_sound(Type::Delete, &player_clone);
                            println!("delete");
                        }
                        _ => { play_sound(Type::Keys, &player_clone); }
                    }
                }

                EventType::ButtonPress(button) => {
                    match button {
                        rdev::Button::Left => {
                            play_sound(Type::LMB, &player_clone);
                        }
                        rdev::Button::Right => {
                            play_sound(Type::RMB, &player_clone);
                        }
                        _ => { }
                    }
                }
                _ => {}
            }
        }).unwrap();

        drop(handle);
    });
}