use serde::Deserialize;
use crate::database::database::{
    database_create_sound_if_missing,
    database_create_sound_effect_if_missing,
    database_sync_sound,
    database_sync_sound_effect
};
use crate::global::global::{PREFIX_FOR_SOUND, PREFIX_FOR_SOUND_EFFECT};
#[derive(Deserialize)]
struct Config {
    id: String,
    effects: Vec<String>
}
const RESOURCES: &str = include_str!("../../ressources/sounds.json");

fn sync_tables() {
    let config: Vec<Config> = serde_json::from_str(RESOURCES).unwrap();

    let expected_sounds: Vec<String> = config
        .iter()
        .map(|s| format!("{}_{}", PREFIX_FOR_SOUND, s.id))
        .collect();

    let expected_effects: Vec<(String, String)> = config
        .iter()
        .flat_map(|s| {
            let sound_id = format!("{}_{}", PREFIX_FOR_SOUND, s.id);
            s.effects.iter().map(move |e| {
                (sound_id.clone(), format!("{}_{}", PREFIX_FOR_SOUND_EFFECT, e))
            })
        })
        .collect();

    let expected_sounds_ref: Vec<&str> = expected_sounds.iter().map(|s| s.as_str()).collect();
    let expected_effects_ref: Vec<(&str, &str)> = expected_effects.iter().map(|(s, e)| (s.as_str(), e.as_str())).collect();

    database_sync_sound(&expected_sounds_ref);
    database_sync_sound_effect(&expected_effects_ref);
}
pub fn init_tables() {
    let config: Vec<Config> = serde_json::from_str(RESOURCES).unwrap();

    for sound in config {
        let sound_id = format!("{}_{}", PREFIX_FOR_SOUND, sound.id);

        database_create_sound_if_missing(&sound_id);

        for effect in &sound.effects {
            let effect_id = format!("{}_{}", PREFIX_FOR_SOUND_EFFECT, effect);

            database_create_sound_effect_if_missing(&sound_id, &effect_id)
        }
    }

    // Delete unused sounds & effects data
    sync_tables();

}