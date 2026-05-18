use std::thread;
use rusqlite::fallible_iterator::FallibleIterator;
use crate::global::global::SOUNDS;
use std::time::Duration;
use crate::functions::sounds::play::function_sound_play;
use crate::functions::sounds::play::function_sound_play::function_sound_play;

pub fn function_sound() {
    thread::spawn(move || {
        loop {
            let list = SOUNDS.get().unwrap();

            for sound in list.lock().unwrap().iter_mut() {
                if sound.play.load(std::sync::atomic::Ordering::Relaxed) {
                    function_sound_play(sound)
                }
            }

            thread::sleep(Duration::from_millis(100));
        }
    });
}