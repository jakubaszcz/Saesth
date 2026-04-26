use std::fs::File;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use rand::RngExt;
use rodio::{Decoder, Player};
use crate::sounds::apply_sound::apply_sound;
use crate::sounds::random_sound;
use crate::utils::sound_stream::SoundEffect;

pub(crate) const FADE_STEPS: u64 = 5;
const FADE_DURATION_MS: u64 = 1500;

pub fn effects_manager(
    effect: SoundEffect,
    play_flag: Arc<AtomicBool>,
    user_volume: Arc<Mutex<f32>>,
    fade_volume: Arc<Mutex<f32>>,
    drift_volume: Arc<Mutex<f32>>,
    mixer: rodio::mixer::Mixer,
) {
    thread::spawn(move || {
        let min = 5;
        let max = 20;

        let min_bonus = 0.2;
        let max_bonus = 0.6;

        while play_flag.load(Ordering::Relaxed) {
            if !effect.data.active.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(200));
                continue;
            }

            let wait = rand::rng().random_range(min..max);
            thread::sleep(Duration::from_secs(wait));

            if !play_flag.load(Ordering::Relaxed) {
                return;
            }

            if !effect.data.active.load(Ordering::Relaxed) {
                continue;
            }

            let path = random_sound::random_sound(effect.path.as_str()) else {
                continue;
            };

            let Ok(file) = File::open(&path) else {
                continue;
            };

            let Ok(source) = Decoder::try_from(file) else {
                continue;
            };

            let player = Player::connect_new(&mixer);

            let bonus = rand::rng().random_range(min_bonus..max_bonus);
            let user = *user_volume.lock().unwrap();
            let fade = *fade_volume.lock().unwrap();
            let drift = *drift_volume.lock().unwrap();

            let volume = (user * fade * drift * bonus).clamp(0.0, 1.0);

            player.set_volume(volume);
            player.append(source);
            player.play();

            loop {
                if !play_flag.load(Ordering::Relaxed)
                    || !effect.data.active.load(Ordering::Relaxed)
                {
                    fade_out_effect(&player, volume);
                    break;
                }

                if player.empty() {
                    break;
                }

                thread::sleep(Duration::from_millis(50));
            }
            player.sleep_until_end();
        }
    });
}

fn fade_out_effect(player: &Player, volume: f32) {
    let steps = FADE_DURATION_MS / FADE_STEPS;

    for step in (0..=steps).rev() {

        let t = step as f32 / steps as f32;
        let eased = t * t;

        player.set_volume(volume * eased);

        thread::sleep(Duration::from_millis(FADE_STEPS));
    }
}