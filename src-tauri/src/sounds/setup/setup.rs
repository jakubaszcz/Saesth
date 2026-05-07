use std::num::{NonZeroU16, NonZeroU32};
use std::sync::{Arc, Mutex, OnceLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use rdev::{listen, Event, EventType};
use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player};
use rodio::buffer::SamplesBuffer;
use crate::types::setup::type_setup::{Setup, SetupDTO, SetupKeys};
use crate::inits::setup::init_setup::init;

static SETUP: OnceLock<Mutex<Setup>> = OnceLock::new();

enum Type {
    Keys,
    Space,
    Delete,
    LMB,
    RMB,
}
fn generate_sound(kind: &Type, sample: f32, setup: &Setup) -> Vec<f32> {

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
            Type::Space => setup.setup_keyboard_volume.lock().unwrap().clone(),
            Type::Delete => setup.setup_keyboard_volume.lock().unwrap().clone(),
            Type::LMB | Type::RMB => setup.setup_mouse_volume.lock().unwrap().clone(),
            Type::Keys => setup.setup_keyboard_volume.lock().unwrap().clone(),
        };

        samples.push(lowpass * ( volume));
    }

    samples
}

fn play_sound(kind: Type, player: &Arc<Mutex<Player>>, setup: &Setup) {


    let volume = match kind {
        Type::Space | Type::Keys | Type::Delete => 0.50,
        Type::LMB | Type::RMB => 0.50,
    };

    let sample = 35000.0;

    let samples = generate_sound(&kind, sample, setup);

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

pub fn setup() {

    SETUP.get_or_init(|| Mutex::new(init()));

    thread::spawn(move || {

        let handle = DeviceSinkBuilder::open_default_sink()
            .expect("failed to open default audio device");

        let player = Arc::new(Mutex::new(
            Player::connect_new(&handle.mixer())
        ));

        let player_clone = player.clone();

        let structure = SETUP.get().unwrap();

        listen(move |event: Event| {

            let setup = structure.lock().unwrap();

            if !setup.setup_global_toggle.load(std::sync::atomic::Ordering::Relaxed) {
                return;
            }

            player_clone.lock().unwrap().set_volume(setup.setup_global_volume.lock().unwrap().clone());

            match event.event_type {

                EventType::KeyPress(key) => {
                    if setup.setup_keyboard_toggle.load(std::sync::atomic::Ordering::Relaxed) {
                        match key {
                            rdev::Key::Space => {
                                play_sound(Type::Space, &player_clone, &setup);
                            }
                            rdev::Key::Delete => {
                                play_sound(Type::Delete, &player_clone, &setup);
                            }
                            _ => {
                                play_sound(Type::Keys, &player_clone, &setup);
                            }
                        }
                    }

                }

                EventType::ButtonPress(button) => {

                    if setup.setup_mouse_toggle.load(std::sync::atomic::Ordering::Relaxed) {
                        match button {
                            rdev::Button::Left => {
                                play_sound(Type::LMB, &player_clone, &setup);
                            }
                            rdev::Button::Right => {
                                play_sound(Type::RMB, &player_clone, &setup);
                            }
                            _ => {}
                        }
                    }
                }

                _ => {}
            }
        }).unwrap();
        drop(handle);
    });
}

pub fn fetch_setup() -> SetupDTO {
    let setup = SETUP
        .get()
        .expect("SETUP is not initialized")
        .lock()
        .unwrap();

    SetupDTO::from(&*setup)
}


pub fn volume_setup(key: SetupKeys, value: f32) {
    let setup = SETUP.get().unwrap().lock().unwrap();

    match key {
        SetupKeys::SetupGlobalVolume => {
            *setup.setup_global_volume.lock().unwrap() = value;
        },
        SetupKeys::SetupKeyboardVolume => {
            *setup.setup_keyboard_volume.lock().unwrap() = value;
        },
        SetupKeys::SetupMouseVolume => {
            *setup.setup_mouse_volume.lock().unwrap() = value;
        },
        SetupKeys::SetupGlobalToggle | SetupKeys::SetupKeyboardToggle | SetupKeys::SetupMouseToggle => todo!()
    }
}

pub fn toggle_setup(key: SetupKeys) {
    let setup = SETUP.get().unwrap().lock().unwrap();

    match key {
        SetupKeys::SetupGlobalToggle => {
            setup.setup_global_toggle.store(!setup.setup_global_toggle.load(Ordering::Relaxed), Ordering::Relaxed);
        },
        SetupKeys::SetupKeyboardToggle => {
            setup.setup_keyboard_toggle.store(!setup.setup_keyboard_toggle.load(Ordering::Relaxed), Ordering::Relaxed);
        },
        SetupKeys::SetupMouseToggle => {
            setup.setup_mouse_toggle.store(!setup.setup_mouse_toggle.load(Ordering::Relaxed), Ordering::Relaxed);
        },
        SetupKeys::SetupGlobalVolume | SetupKeys::SetupKeyboardVolume | SetupKeys::SetupMouseVolume => todo!()
    }
}