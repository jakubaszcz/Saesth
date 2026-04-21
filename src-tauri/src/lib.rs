use std::collections::HashMap;
use std::fs::File;
use std::sync::{Mutex, OnceLock};
use rodio::{Decoder, DeviceSinkBuilder, MixerDeviceSink, Source};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SoundData {
    pub play: bool,
    pub path: String,
    pub volume: f32
}

pub struct SoundStream {
    pub handle: Option<MixerDeviceSink>,
    pub data: SoundData
}

pub type SoundMap = HashMap<String, SoundStream>;

static SOUND_MAP: OnceLock<Mutex<SoundMap>> = OnceLock::new();
fn init_sounds() {
    let mut map = SoundMap::new();
    map.insert("rain".to_string(), SoundStream {
        handle: None,
        data: SoundData {
            play: false,
            volume: 0.5,
            path: "sounds/rain.mp3".to_string(),
        }
    });
    map.insert("thunder".to_string(), SoundStream {
        handle: None,
        data: SoundData {
            play: false,
            volume: 0.5,
            path: "sounds/thunder.mp3".to_string(),
        }
    });

    SOUND_MAP.get_or_init(|| Mutex::new(map));
}

#[tauri::command]
fn get_sounds() -> HashMap<String, SoundData> {
    let map = SOUND_MAP.get().unwrap().lock().unwrap();
    map.iter().map(|(k, v)| (k.clone(), v.data.clone())).collect()
}

#[tauri::command]
fn change_volume(id: String, volume: f32) -> HashMap<String, SoundData> {
    let mut map = SOUND_MAP.get().unwrap().lock().unwrap();

    if let Some(sound) = map.get_mut(&id) {
        sound.data.volume = volume;
    }
    map.iter()
        .map(|(k, v)| (k.clone(), v.data.clone()))
        .collect()
}

#[tauri::command]
fn toggle_play(id: String) -> HashMap<String, SoundData> {
    let mut map = SOUND_MAP.get().unwrap().lock().unwrap();

    if let Some(sound) = map.get_mut(&id) {
        if sound.data.play {
            if let Some(handle) = sound.handle.take() {
                drop(handle);
            }
            sound.data.play = false;
        } else {
            let handle = DeviceSinkBuilder::open_default_sink()
                .expect("failed to open default audio device");

            let file = File::open(&sound.data.path)
                .expect("failed to open audio file");

            let source = Decoder::try_from(file)
                .expect("failed to decode audio file")
                .amplify(sound.data.volume)
                .repeat_infinite();

            handle.mixer().add(source);

            sound.handle = Some(handle);
            sound.data.play = true;
        }
    }

    map.iter()
        .map(|(k, v)| (k.clone(), v.data.clone()))
        .collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    init_sounds();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_sounds,
            toggle_play,
            change_volume,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}