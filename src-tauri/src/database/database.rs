use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use rusqlite::Connection;
use rusqlite::fallible_iterator::FallibleIterator;
use std::sync::{Mutex, OnceLock};

static DATABASE: OnceLock<Mutex<Connection>> = OnceLock::new();

fn db() -> std::sync::MutexGuard<'static, Connection> {
    DATABASE
        .get()
        .unwrap()
        .lock()
        .unwrap()
}

pub fn init_database_settings() {
    create_setting_if_missing("close_to_tray", "true");
}

pub fn set_setting(key: &str, value: &str) {
    let conn = db();

    conn.execute(
        "INSERT INTO settings (id, value)
         VALUES (?1, ?2)
         ON CONFLICT(id)
         DO UPDATE SET value = excluded.value",
        rusqlite::params![key, value],
    ).unwrap();
}

pub fn get_setting(key: &str) -> String {
    let conn = db();

    conn.query_row(
        "SELECT value FROM settings WHERE id = ?1",
        [key],
        |row| row.get(0),
    ).unwrap_or_else(|_| "".to_string())
}

fn get_database_path() -> PathBuf {
    let directory = ProjectDirs::from("com", "saesth", "saesth").unwrap();

    let local = directory.data_dir();
    fs::create_dir_all(local).unwrap();


    local.join("database.db")
}

pub fn init_db() {
    let path = get_database_path();
    let conn = Connection::open(path).unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS settings (
            id TEXT PRIMARY KEY,
            value TEXT
        )", []).unwrap();

    DATABASE.set(Mutex::new(conn)).unwrap();
}

pub fn database_create_sound_table_if_missing() {
    let conn = db();

    conn.execute("CREATE TABLE IF NOT EXISTS sounds (
            id TEXT PRIMARY KEY,
            volume REAL
        )
        ",[]).unwrap();
}

pub fn database_create_sound_effect_table_if_missing() {
    let conn = db();

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

    let conn = db();

    conn.execute(
        "INSERT OR IGNORE INTO effects (effect, sound, active) VALUES (?1, ?2, 0)",
        rusqlite::params![effect, sound],
    ).unwrap();
}

pub fn database_sync_sound_effect(expected_effects: &[(&str, &str)]) {
    let conn = db();

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

pub fn get_effect_active(sound: &str, effect: &str) -> bool {
    let conn = db();

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

pub fn database_create_sound_if_missing(sound: &str) {
    database_create_sound_table_if_missing();

    let conn = db();

    conn.execute(
        "INSERT OR IGNORE INTO sounds (id, volume) VALUES (?1, 0.5)",
        [sound],
    ).unwrap();
}

pub fn database_sync_sound(expected_sounds: &[&str]) {

    let conn = db();

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

pub fn create_setting_if_missing(key: &str, value: &str) {
    let conn = db();

    conn.execute(
        "INSERT OR IGNORE INTO settings (id, value) VALUES (?1, ?2)",
        rusqlite::params![key, value],
    ).unwrap();
}

pub fn get_volume(sound: &str) -> f32 {
    let conn = db();

    conn.query_row(
        "SELECT volume FROM sounds WHERE id = ?1",
        [sound],
        |row| row.get(0),
    ).unwrap_or(0.5)
}

pub fn set_volume(sound: &str, volume: f32) {
    let conn = db();

    conn.execute(
        "UPDATE sounds SET volume = ?1 WHERE id = ?2",
        rusqlite::params![volume, sound],
    ).unwrap();
}