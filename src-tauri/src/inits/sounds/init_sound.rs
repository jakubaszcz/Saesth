use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use rusqlite::fallible_iterator::FallibleIterator;
use serde::Deserialize;
use crate::global::global::{PREFIX_FOR_SOUND, PREFIX_FOR_SOUND_EFFECT};
use crate::database::database::{database_get_sound_effect_active, database_get_sound_volume};
use crate::inits::sounds::init_tables_sound::init_tables;
use crate::types::sounds::type_sounds::{Sound, Effect};

#[derive(Deserialize)]
struct Config {
    id: String,
    effects: Vec<String>
}
const RESOURCES: &str = include_str!("../../ressources/sounds.json");
fn make_stream(id: &str, effects: Vec<Effect>) -> Sound {

    let sound_id = format!("{}_{}", PREFIX_FOR_SOUND, id);

    Sound {
        sound_id: sound_id.clone(),
        handle: None,
        player: None,
        play: Arc::new(AtomicBool::new(false)),
        volume: Arc::new(Mutex::new(database_get_sound_volume(&sound_id.clone()))),
        fade_volume: Arc::new(Mutex::new(1.0)),
        drift_volume: Arc::new(Mutex::new(1.0)),
        effects
    }
}

fn make_effect(sound_id: &str, id: &str) -> Effect {

    let effect_id = format!("{}_{}", PREFIX_FOR_SOUND_EFFECT, id);

    Effect {
        effect_id: effect_id.clone(),
        active: Arc::new(AtomicBool::new(
            database_get_sound_effect_active(format!("{}_{}", PREFIX_FOR_SOUND, sound_id).as_str(), effect_id.clone().as_str()))
        ),
    }
}
pub fn init() -> Vec<Sound> {

    {
        init_tables();
    }

    let config: Vec<Config> = serde_json::from_str(RESOURCES).unwrap();

    config
        .iter()
        .map(|sound| {
            let effects = sound.effects
                .iter()
                .map(|effect| make_effect(&sound.id, effect))
                .collect();

            make_stream(&sound.id, effects)
        })
        .collect()
}