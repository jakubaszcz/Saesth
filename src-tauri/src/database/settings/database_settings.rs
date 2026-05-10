use crate::global::global::global_database_get;

pub fn database_create_settings_table_if_missing() {
    let conn = global_database_get();

    conn.execute("CREATE TABLE IF NOT EXISTS settings (
            id TEXT PRIMARY KEY,
            active  INTEGER NOT NULL DEFAULT 0
    ", []).unwrap();
}

pub fn database_create_setting_if_missing(setting: &str) {
    database_create_settings_table_if_missing();

    let conn = global_database_get();

    conn.execute(
        "INSERT OR IGNORE INTO settings (id, active) VALUES (?1, 0)",
        [setting],
    ).unwrap();

}

pub fn database_sync_setting(expected_settings: &[&str]) {
    let conn = global_database_get();

    let expected_sounds_list = expected_settings
        .iter()
        .map(|s| format!("'{}'", s))
        .collect::<Vec<_>>()
        .join(", ");

    conn.execute_batch(&format!(
        "DELETE FROM settings WHERE id NOT IN ({})",
        expected_sounds_list
    )).unwrap();
}

pub fn database_get_setting_active(setting: &str) -> bool {
    let conn = global_database_get();

    conn.query_row(
        "SELECT active FROM settings WHERE id = ?1",
        [setting],
        |row| {
            let active: i32 = row.get(0)?;
            Ok(active == 1)
        },
    ).unwrap_or(false)
}