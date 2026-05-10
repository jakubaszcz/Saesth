use crate::global::global::global_database_get;

pub fn database_create_sound_table_if_missing() {
    let conn = global_database_get();

    conn.execute("CREATE TABLE IF NOT EXISTS sounds (
            id TEXT PRIMARY KEY,
            volume REAL
        )
        ",[]).unwrap();
}

pub fn database_create_sound_if_missing(sound: &str) {
    database_create_sound_table_if_missing();

    let conn = global_database_get();

    conn.execute(
        "INSERT OR IGNORE INTO sounds (id, volume) VALUES (?1, 0.5)",
        [sound],
    ).unwrap();
}

pub fn database_sync_sound(expected_sounds: &[&str]) {
    let conn = global_database_get();

    let expected_sounds_list = expected_sounds
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<_>>()
        .join(", ");

    conn.execute_batch(&format!(
        "DELETE FROM sounds WHERE id NOT IN ({})",
        expected_sounds_list
    )).unwrap();
}

pub fn database_get_sound_volume(sound: &str) -> f32 {
    let conn = global_database_get();

    conn.query_row(
        "SELECT volume FROM sounds WHERE id = ?1",
        [sound],
        |row| row.get(0),
    ).unwrap_or(0.5)
}

pub fn database_set_sound_volume(sound: &str, volume: f32) {
    let conn = global_database_get();

    conn.execute(
        "UPDATE sounds SET volume = ?1 WHERE id = ?2",
        rusqlite::params![volume, sound],
    ).unwrap();
}







pub fn database_create_sound_effect_table_if_missing() {
    let conn = global_database_get();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS effects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            effect TEXT NOT NULL,
            sound TEXT KEY NOT NULL,
            active INTEGER NOT NULL DEFAULT 0,

            FOREIGN KEY (sound) REFERENCES sounds(id) ON DELETE CASCADE,
            UNIQUE (effect, sound)
        )",
        []
    ).unwrap();
}

pub fn database_create_sound_effect_if_missing(sound: &str, effect: &str) {

    database_create_sound_effect_table_if_missing();

    let conn = global_database_get();

    conn.execute(
        "INSERT OR IGNORE INTO effects (effect, sound, active) VALUES (?1, ?2, 0)",
        rusqlite::params![effect, sound],
    ).unwrap();
}

pub fn database_sync_sound_effect(expected_effects: &[(&str, &str)]) {
    let conn = global_database_get();

    let expected_effects_list = expected_effects
        .iter()
        .map(|(sound, effect)| format!("('{}', '{}')", sound, effect))
        .collect::<Vec<_>>()
        .join(", ");

    conn.execute_batch(&format!(
        "DELETE FROM effects WHERE (sound, effect) NOT IN ({})",
        expected_effects_list
    )).unwrap();
}

pub fn database_get_sound_effect_active(sound: &str, effect: &str) -> bool {
    let conn = global_database_get();

    conn.query_row(
        "SELECT active FROM effects
         WHERE sound = ?1 AND effect = ?2",
        rusqlite::params![sound, effect],
        |row| {
            let active: i32 = row.get(0)?;
            Ok(active == 1)
        },
    ).unwrap_or(false)
}