use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use rodio::{MixerDeviceSink, Player};
use rodio::cpal::Data;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Structures {
    pub id: String,
    pub play: bool,
    pub path: String,
    pub volume: f32
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]

pub struct SoundFront {
    pub data: Structures,
    pub effects: Vec<SoundEffectFront>,
}
#[derive(Clone, serde::Serialize, serde::Deserialize)]

pub struct SoundEffectFront {
    pub id: String,
    pub active: bool,
}

pub struct SoundStream {

    pub effects: Vec<SoundEffect>,

    pub handle: Option<MixerDeviceSink>,
    pub player: Option<Arc<Mutex<Player>>>,
    pub play: Arc<AtomicBool>,
    pub volume: Arc<Mutex<f32>>,
    pub fade_volume: Arc<Mutex<f32>>,
    pub drift_volume: Arc<Mutex<f32>>,
    pub data: Structures
}

#[derive(Clone)]
pub struct SoundEffect {
    pub player: Option<Arc<Mutex<Player>>>,
    pub path: String,
    pub data: SoundEffectData
}

#[derive(Clone)]
pub struct SoundEffectData {
    pub id: String,
    pub active: Arc<AtomicBool>,
}

pub type SoundList = Vec<SoundStream>;