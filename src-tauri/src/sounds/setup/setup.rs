use std::num::{NonZeroU16, NonZeroU32};
use std::sync::{Arc, Mutex, OnceLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use rdev::{listen, Event, EventType};
use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player};
use rodio::buffer::SamplesBuffer;
use crate::sounds::random_sound;
use crate::utils;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupUtilities {
    Setup,
    Keyboard,
    Mouse,
}

struct Setup {
    toggle: Arc<AtomicBool>,
    volume: Arc<Mutex<f32>>,
    keyboard_toggle: Arc<AtomicBool>,
    keyboard_volume: Arc<Mutex<f32>>,
    mouse_toggle: Arc<AtomicBool>,
    mouse_volume: Arc<Mutex<f32>>,
}

#[derive(serde::Serialize)]
pub struct SetupDTO {
    pub toggle: bool,
    pub volume: f32,
    pub keyboard_toggle: bool,
    pub keyboard_volume: f32,
    pub mouse_toggle: bool,
    pub mouse_volume: f32,
}

impl From<&Setup> for SetupDTO {
    fn from(setup: &Setup) -> Self {
        Self {
            toggle: setup.toggle.load(Ordering::Relaxed),
            volume: *setup.volume.lock().unwrap(),
            keyboard_toggle: setup.keyboard_toggle.load(Ordering::Relaxed),
            keyboard_volume: *setup.keyboard_volume.lock().unwrap(),
            mouse_toggle: setup.mouse_toggle.load(Ordering::Relaxed),
            mouse_volume: *setup.mouse_volume.lock().unwrap(),
        }
    }
}

static SETUP: OnceLock<Mutex<Setup>> = OnceLock::new();

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

fn init_setup() {
    SETUP.get_or_init(|| {
        Mutex::new(Setup {
            toggle: Arc::new(AtomicBool::new(true)),
            volume: Arc::new(Mutex::new(0.5)),
            keyboard_toggle: Arc::new(AtomicBool::new(true)),
            keyboard_volume: Arc::new(Mutex::new(0.5)),
            mouse_toggle: Arc::new(AtomicBool::new(true)),
            mouse_volume: Arc::new(Mutex::new(0.5)),
        })
    });
}

pub fn setup() {

    init_setup();

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

            if !setup.toggle.load(std::sync::atomic::Ordering::Relaxed) {
                return;
            }

            match event.event_type {

                EventType::KeyPress(key) => {
                    if setup.keyboard_toggle.load(std::sync::atomic::Ordering::Relaxed) {
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

                }

                EventType::ButtonPress(button) => {

                    if setup.mouse_toggle.load(std::sync::atomic::Ordering::Relaxed) {
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


pub fn volume_setup(utils: SetupUtilities, volume: f32) {
    let setup = SETUP.get().unwrap();

    match utils {
        SetupUtilities::Setup => {
            let mut setup = setup.lock().unwrap();
            *setup.volume.lock().unwrap() = volume;
        },
        SetupUtilities::Keyboard => {
            let mut setup = setup.lock().unwrap();
            *setup.keyboard_volume.lock().unwrap() = volume;
        },
        SetupUtilities::Mouse => {
            let mut setup = setup.lock().unwrap();
            *setup.mouse_volume.lock().unwrap() = volume;
        }
    }
}

pub fn toggle_setup(utils: SetupUtilities) {
    let setup = SETUP.get().unwrap();

    match utils {
        SetupUtilities::Setup => {
            println!("toggled");
            let mut setup = setup.lock().unwrap();
            setup.toggle.store(!setup.toggle.load(Ordering::Relaxed), Ordering::Relaxed);
        },
        SetupUtilities::Keyboard => {
            let mut setup = setup.lock().unwrap();
            setup.keyboard_toggle.store(!setup.keyboard_toggle.load(Ordering::Relaxed), Ordering::Relaxed);
        },
        SetupUtilities::Mouse => {
            let mut setup = setup.lock().unwrap();
            setup.mouse_toggle.store(!setup.mouse_toggle.load(Ordering::Relaxed), Ordering::Relaxed);
        }
    }
}