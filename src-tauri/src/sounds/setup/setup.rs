use std::num::{NonZeroU16, NonZeroU32};
use std::sync::{Arc, Mutex, OnceLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use rdev::{listen, Event, EventType};
use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player};
use rodio::buffer::SamplesBuffer;
use crate::sounds::random_sound;
use crate::utils;

static TOGGLE_SETUP: AtomicBool = AtomicBool::new(true);

enum Type {
    Keys,
    Space,
    Delete,
    LMB,
    RMB,
}
fn generate_sound(kind: &Type, sample: f32) -> Vec<f32> {
    let sample_rate = sample;
    let duration = match kind {
        Type::Space => 0.075,
        Type::Delete => 0.055,
        _ => 0.045,
    };

    let count = (sample_rate * duration) as usize;
    let mut samples = Vec::with_capacity(count);

    let base_freq = match kind {
        Type::Keys => 600.0,
        Type::Space => 420.0,
        Type::Delete => 700.0,
        Type::LMB => 1200.0,
        Type::RMB => 700.0,
    };

    // Pitch [0.85, 1.10]
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
            Type::Space => 0.45,
            Type::Delete => 0.38,
            Type::LMB | Type::RMB => 0.32,
            Type::Keys => 0.28,
        };

        samples.push(lowpass * volume);
    }

    samples
}

fn play_sound(kind: Type, player: &Arc<Mutex<Player>>) {

    let volume = match kind {
        Type::Space | Type::Keys | Type::Delete => 0.50,
        Type::LMB | Type::RMB => 0.50,
    };

    let sample = 35000.0;

    let samples = generate_sound(&kind, sample);

    let source = SamplesBuffer::new(
        NonZeroU16::new(1).unwrap(),
        NonZeroU32::new(sample as u32).unwrap(),
        samples,
    );

    let mut player = player.lock().unwrap();

    player.stop();
    player.set_volume(volume);
    player.append(source);
    player.play();
}

pub fn setup() {
    thread::spawn(move || {

        let handle = DeviceSinkBuilder::open_default_sink()
            .expect("failed to open default audio device");

        let player = Arc::new(Mutex::new(
            Player::connect_new(&handle.mixer())
        ));

        let player_clone = player.clone();

        listen(move |event: Event| {

            if !TOGGLE_SETUP.load(std::sync::atomic::Ordering::Relaxed) {
                return;
            }

            match event.event_type {
                EventType::KeyPress(key) => {
                    match key {
                        rdev::Key::Space => {
                            play_sound(Type::Space, &player_clone);
                        }
                        rdev::Key::Delete => {
                            play_sound(Type::Delete, &player_clone);
                        }
                        _ => {
                            play_sound(Type::Keys, &player_clone);
                        }
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
                        _ => {}
                    }
                }

                _ => {}
            }
        }).unwrap();
        drop(handle);
    });
}

pub fn toggle_setup() {
    let current = TOGGLE_SETUP.load(Ordering::Relaxed);
    TOGGLE_SETUP.store(!current, Ordering::Relaxed);
}