use std::num::{NonZeroU16, NonZeroU32};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{Ordering};
use std::sync::mpsc::Sender;
use std::thread;
use rdev::{listen, Event, EventType};
use rodio::{DeviceSinkBuilder, Player};
use rodio::buffer::SamplesBuffer;
use crate::global::global::{PACK, PREFIX_FOR_SETUP, SETUP};
use crate::types::manifest::type_manifest::ManifestSetup;
use crate::utils::prefix::util_prefix::util_prefix_add_prefix;

enum Type {
    Keys,
    Space,
    Delete,
    LMB,
    RMB,
}

const GLOBAL: &str = "global";
const KEYBOARD: &str = "keyboard";
const MOUSE: &str = "mouse";

fn function_setup_get_setup_toggled(compare: String) -> bool {
    let list = match SETUP.get() {
        Some(l) => l,
        None => return false,
    };

    list.lock()
        .unwrap()
        .iter()
        .find(|setup| setup.setup_id == compare)
        .map(|setup| setup.toggle.load(Ordering::Relaxed))
        .unwrap_or(false)
}

fn function_setup_get_setup_volume(compare: String) -> f32 {
    let list = match SETUP.get() {
        Some(l) => l,
        None => return 0.0,
    };

    list.lock()
        .unwrap()
        .iter()
        .find(|setup| setup.setup_id == compare)
        .map(|setup| *setup.volume.lock().unwrap())
        .unwrap_or(0.0)
}

fn generate_sound(kind: &Type, setup: &ManifestSetup) -> Vec<f32> {
    let sample_rate = setup.global.sample_rate;
    let duration = match kind {
        Type::Space => 0.075,
        Type::Delete => 0.055,
        _ => 0.045,
    };

    let count = (sample_rate * duration) as usize;
    let mut samples = Vec::with_capacity(count);

    let base_freq = match kind {
        Type::Keys => setup.keyboard.frequency,
        Type::Space => setup.keyboard.frequency,
        Type::Delete => setup.keyboard.frequency,
        Type::LMB => setup.mouse.frequency,
        Type::RMB => setup.mouse.frequency,
    };

    let pitch = 0.85 + rand::random::<f32>() * 0.2;
    let freq = base_freq * pitch;

    let mut lowpass = 0.0;

    for i in 0..count {
        let t = i as f32 / sample_rate;
        let x = i as f32 / count as f32;

        let attack = (x / 0.08).min(1.0);

        let decay = (-7.5 * x).exp();

        let envelope = attack * decay;

        let tone = (2.0 * std::f32::consts::PI * freq * t).sin() * 0.28;
        let harmonic = (2.0 * std::f32::consts::PI * freq * 2.0 * t).sin() * 0.06;

        let noise = (rand::random::<f32>() - 0.5) * 0.18;

        let transient = if i < 10 {
            (1.0 - i as f32 / 10.0) * 0.14
        } else {
            0.0
        };

        let raw = (tone + harmonic + noise + transient) * envelope;

        lowpass += 0.12 * (raw - lowpass);

        let volume = match kind {
            Type::Space => setup.keyboard.volume,
            Type::Delete => setup.keyboard.volume,
            Type::LMB | Type::RMB => setup.mouse.volume,
            Type::Keys => setup.keyboard.volume,
        };

        samples.push(lowpass * ( volume));
    }

    samples
}

fn play_sound(kind: Type, player: &Arc<Mutex<Player>>) {

    let setup = {
        let Some(pack_mutex) = PACK.get() else {
            return;
        };

        let pack = pack_mutex.lock().unwrap();

        if pack.id.is_empty() {
            return;
        }

        pack.setup.clone()
    };
    let sample = setup.global.sample_rate;

    let samples = generate_sound(&kind, &setup);

    let source = SamplesBuffer::new(
        NonZeroU16::new(1).unwrap(),
        NonZeroU32::new(sample as u32).unwrap(),
        samples,
    );

    let player = player.lock().unwrap();

    player.stop();
    player.append(source);
    player.play();
}

pub fn function_setup_init() {
    let (tx, rx) = std::sync::mpsc::channel::<Type>();

    thread::spawn(move || {
        let handle = DeviceSinkBuilder::open_default_sink()
            .expect("failed to open default audio device");

        let player = Arc::new(Mutex::new(
            Player::connect_new(&handle.mixer())
        ));

        while let Ok(kind) = rx.recv() {
            if !function_setup_get_setup_toggled(util_prefix_add_prefix(PREFIX_FOR_SETUP, GLOBAL)) {
                continue;
            }

            player.lock().unwrap().set_volume(
                function_setup_get_setup_volume(util_prefix_add_prefix(PREFIX_FOR_SETUP, GLOBAL))
            );

            play_sound(kind, &player);
        }
        drop(handle);
    });

    thread::spawn(move || {
        key_event(tx)
    });
}

#[cfg(target_os = "linux")]
fn key_event(tx: Sender<Type>) {

}

#[cfg(target_os = "windows")]
fn key_event(tx: Sender<Type>) {

    let mut pressed_keys = std::collections::HashSet::new();
    let mut pressed_buttons = std::collections::HashSet::new();

    listen(move |event: Event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                if pressed_keys.insert(key) {
                    if function_setup_get_setup_toggled(util_prefix_add_prefix(PREFIX_FOR_SETUP, KEYBOARD)) {
                        let kind = match key {
                            rdev::Key::Space => Some(Type::Space),
                            rdev::Key::Delete => Some(Type::Delete),
                            _ => Some(Type::Keys),
                        };
                        if let Some(k) = kind {
                            let _ = tx.send(k);
                        }
                    }
                }
            }

            EventType::KeyRelease(key) => {
                pressed_keys.remove(&key);
            }

            EventType::ButtonPress(button) => {
                if pressed_buttons.insert(button) {
                    if function_setup_get_setup_toggled(util_prefix_add_prefix(PREFIX_FOR_SETUP, MOUSE)) {
                        let kind = match button {
                            rdev::Button::Left => Some(Type::LMB),
                            rdev::Button::Right => Some(Type::RMB),
                            _ => None,
                        };
                        if let Some(k) = kind {
                            let _ = tx.send(k);
                        }
                    }
                }
            }
            EventType::ButtonRelease(button) => {
                pressed_buttons.remove(&button);
            }
            _ => {}
        }
    }).expect("failed to listen to events");
}