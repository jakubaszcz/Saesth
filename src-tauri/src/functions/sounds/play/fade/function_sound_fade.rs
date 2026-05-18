use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use std::thread;
use rodio::Player;
use crate::functions::sounds::utils::function_sound_util_volume::function_sound_util_volume;

const FADE_STEPS: u64 = 5;
const FADE_DURATION_MS: u64 = 1500;
pub fn function_sound_fade(
    flag: Arc<AtomicBool>,
    player: Arc<Mutex<Player>>,
    user_volume: Arc<Mutex<f32>>,
    fade_volume: Arc<Mutex<f32>>,
    drift_volume: Arc<Mutex<f32>>,
) {
    thread::spawn(move || {
        let steps = FADE_DURATION_MS / FADE_STEPS;

        for step in 0..=steps {
            if !flag.load(std::sync::atomic::Ordering::Relaxed) {
                function_sound_util_volume(
                    &player.clone(),
                    &user_volume,
                    &fade_volume,
                    &drift_volume,
                );
                return;
            }

            let t = step as f32 / steps as f32;
            let eased = t * t;

            *fade_volume.lock().unwrap() = eased;

            function_sound_util_volume(
                &player.clone(),
                &user_volume,
                &fade_volume,
                &drift_volume,
            );

            thread::sleep(std::time::Duration::from_millis(FADE_STEPS));
        }
    });
}