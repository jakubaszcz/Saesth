use std::sync::atomic::Ordering;
use crate::global::global::SOUNDS;
use crate::types::sounds::type_sounds::SoundDTO;

pub fn commands_sounds_fetch_sounds() -> Vec<SoundDTO> {
    let list = SOUNDS.get().unwrap().lock().unwrap();

    list.iter()
        .map(|sound| SoundDTO::from(sound))
        .collect()
}

pub fn commands_sounds_toggle_sound(sound_id: String) -> bool {
    let list = SOUNDS.get().unwrap().lock().unwrap();

    list.iter()
        .find(|s| s.sound_id == sound_id)
        .map(|s| {
            let new_val = !s.play.load(Ordering::Relaxed);
            s.play.store(new_val, Ordering::Relaxed);
            new_val
        })
        .unwrap_or(false)
}