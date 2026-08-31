use std::fs::File;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use rand::RngExt;
use rodio::{Decoder, Player};
use crate::functions::sounds::utils::function_random_sound::function_random_sound;
use crate::types::sounds::type_sounds::Effect;
use crate::utils::prefix::util_prefix::util_prefix_remove_prefix;

pub(crate) const FADE_STEPS: u64 = 5;
const FADE_DURATION_MS: u64 = 1500;

const caca: Vec<String> = vec![];
const PATH: &str = "./sounds/effects";

pub fn function_sound_effect(
    effect: Effect,
    play_flag: Arc<AtomicBool>,
    user_volume: Arc<Mutex<f32>>,
    fade_volume: Arc<Mutex<f32>>,
    drift_volume: Arc<Mutex<f32>>,
    mixer: rodio::mixer::Mixer,
) {
    thread::spawn(move || {
        let min = 1;
        let max = 5;

        let min_bonus = 0.2;
        let max_bonus = 0.6;

        while play_flag.load(Ordering::Relaxed) {
            if !effect.active.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(200));
                continue;
            }

            let wait = rand::rng().random_range(min..max);
            thread::sleep(Duration::from_secs(wait));

            println!("effect playing : {:?}", util_prefix_remove_prefix(&effect.effect_id).as_str());

            if !play_flag.load(Ordering::Relaxed) {
                return;
            }

            if !effect.active.load(Ordering::Relaxed) {
                continue;
            }

            let exe_path = std::env::current_exe().unwrap();
            let base_path = exe_path.parent().unwrap();
            
            // Check if we are in development or production
            let sounds_path = if base_path.join("sounds").exists() {
                base_path.join("sounds")
            } else {
                base_path.join("../sounds")
            };
            
            let effects_path = sounds_path.join("effects");

            let path = effects_path
                .join(util_prefix_remove_prefix(&effect.effect_id).as_str());

            let sound_file_path = function_random_sound(path.to_str().unwrap());
            if sound_file_path.as_os_str().is_empty() {
                continue;
            }

            let Ok(file) = File::open(sound_file_path) else {
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
                    || !effect.active.load(Ordering::Relaxed)
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


