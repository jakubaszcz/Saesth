use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use rodio::{MixerDeviceSink, Player};

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