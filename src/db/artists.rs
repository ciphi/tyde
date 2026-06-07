use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result, params};

#[derive(Debug)]
pub(crate) struct Artist {
    id: i64,
    /// Canonical name
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub fn insert(conn: &Connection, name: &str) -> Result<Artist> {
    let now = Utc::now();
    conn.execute(
        "INSERT INTO artists (name, created_at, updated_at) VALUES (?1, ?2, ?3)",
        params![name, now, now],
    )?;
    let id = conn.last_insert_rowid();
    Ok(Artist {
        id,
        name: name.to_string(),
        created_at: now,
        updated_at: now,
    })
}
