use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use rodio::{MixerDeviceSink, Player};
use crate::types::setup::type_setup::{Setup, SetupDTO};

pub struct Sound {

    pub sound_id: String,

    pub handle: Option<MixerDeviceSink>,
    pub player: Option<Arc<Mutex<Player>>>,

    pub play: Arc<AtomicBool>,
    pub volume: Arc<Mutex<f32>>,
    pub fade_volume: Arc<Mutex<f32>>,
    pub drift_volume: Arc<Mutex<f32>>,

    pub effects: Vec<Effect>,
}

pub struct Effect {
    pub effect_id: String,
    pub active: Arc<AtomicBool>,
}

#[derive(serde::Serialize)]
pub struct SoundDTO {
    sound_id: String,
    play: bool,
    volume: f32,
    effects: Vec<EffectDTO>,
}

#[derive(serde::Serialize)]
pub struct EffectDTO {
    effect_id: String,
    active: bool,
}

impl From<&Sound> for SoundDTO {
    fn from(sound: &Sound) -> Self {
        Self {
            sound_id: sound.sound_id.clone(),
            play: sound.play.load(Ordering::Relaxed),
            volume: *sound.volume.lock().unwrap(),
            effects: sound.effects.iter().map(|effect| EffectDTO {
                effect_id: effect.effect_id.clone(),
                active: effect.active.load(Ordering::Relaxed),
            }).collect(),
        }
    }
}