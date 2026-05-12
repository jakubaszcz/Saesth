use crate::global::global::global_database_get;

pub fn database_create_setup_table_if_missing() {
    let conn = global_database_get();

    conn.execute("CREATE TABLE IF NOT EXISTS setup (
            id TEXT PRIMARY KEY,
            toggle INTEGER NOT NULL DEFAULT 0
            volume INTEGER NOT NULL DEFAULT 0
            )
    ", []).unwrap();
}

pub fn database_create_setup_if_missing(setting: &str) {
    database_create_setup_table_if_missing();

    let conn = global_database_get();

    conn.execute(
        "INSERT OR IGNORE INTO setup (id, toggle, volume) VALUES (?1, 0, 0.5)",
        [setting],
    ).unwrap();
}

pub fn database_sync_setup(expected_setup: &[&str]) {
    let conn = global_database_get();

    let expected_sounds_list = expected_setup
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<_>>()
        .join(", ");

    conn.execute_batch(&format!(
        "DELETE FROM sounds WHERE id NOT IN ({})",
        expected_sounds_list
    )).unwrap();
}

pub fn database_get_setup_toggle(setting: &str) -> bool {
    let conn = global_database_get();

    conn.query_row(
        "SELECT toggle FROM setup WHERE id = ?1",
        [setting],
        |row| {
            let toggle: i32 = row.get(0)?;
            Ok(toggle == 1)
        },
    ).unwrap_or(false)
}

pub fn database_get_setup_volume(setting: &str) -> f32 {
    let conn = global_database_get();

    conn.query_row(
        "SELECT volume FROM setup WHERE id = ?1",
        [setting],
        |row| row.get(0),
    ).unwrap_or(0.5)
}