use crate::global::global::global_database_get;

pub fn database_create_pack_table_if_missing() {
    let conn = global_database_get();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS pack (
            id TEXT PRIMARY KEY
        )",
        [],
    ).unwrap();
}

pub fn database_pack_get_active_pack() -> String {
    let conn = global_database_get();

    conn.query_row(
        "SELECT id FROM pack LIMIT 1",
        [],
        |row| row.get::<_, String>(0),
    )
        .unwrap_or(String::new())
}

pub fn  database_pack_set_active_pack(id: String) {
    let conn = global_database_get();

    conn.execute(
        "DELETE FROM pack",
        [],
    ).unwrap();

    conn.execute(
        "INSERT INTO pack (id) VALUES (?1)",
        rusqlite::params![id],
    ).unwrap();
}