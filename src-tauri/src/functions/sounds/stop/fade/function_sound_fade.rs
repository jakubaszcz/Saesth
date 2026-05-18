use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use rodio::{MixerDeviceSink, Player};
use crate::functions::sounds::utils::function_sound_util_volume::function_sound_util_volume;

const FADE_STEPS: u64 = 5;
const FADE_DURATION_MS: u64 = 1500;

pub fn function_sound_fade(
    flag: Arc<AtomicBool>,
    player: Arc<Mutex<Player>>,
    user_volume: Arc<Mutex<f32>>,
    fade_volume: Arc<Mutex<f32>>,
    drift_volume: Arc<Mutex<f32>>,
    handle: MixerDeviceSink,
) {
    thread::spawn(move || {
        let start_volume = *fade_volume.lock().unwrap();
        let start_t = start_volume.sqrt(); // because eased = t * t

        let steps = FADE_DURATION_MS / FADE_STEPS;
        let start_step = (start_t * steps as f32).round() as u64;

        for step in (0..=start_step).rev() {
            if flag.load(Ordering::Relaxed) {
                function_sound_util_volume(
                    &player,
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
                &player,
                &user_volume,
                &fade_volume,
                &drift_volume,
            );

            thread::sleep(Duration::from_millis(FADE_STEPS));
        }

        *fade_volume.lock().unwrap() = 0.0;

        function_sound_util_volume(
            &player,
            &user_volume,
            &fade_volume,
            &drift_volume,
        );

        drop(handle);
    });
}