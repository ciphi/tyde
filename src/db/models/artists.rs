use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result, params};
use tracing::{info, instrument};

use crate::positive;

#[derive(Debug)]
pub(crate) struct Artist {
    pub id: i64,
    /// Canonical name
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[instrument(name = "add", skip_all)]
pub fn insert(conn: &Connection, name: &str) -> Result<Artist> {
    let now = Utc::now();

    conn.execute(
        "INSERT INTO artists (name, created_at, updated_at) VALUES (?1, ?2, ?3)",
        params![name, now, now],
    )?;

    let id = conn.last_insert_rowid();

    println!("Added artist: {}", positive!("{}", name));
    info!(artist_name = name, created_at = now.to_string());

    // ++    let s = now.format("%Y-%m-%d %H:%M:%S").to_string();
    // ++    let naive = NaiveDateTime::parse_from_str("2026-06-07 14:35:22", "%Y-%m-%d %H:%M:%S")?;
    // ++
    // ++    let created_at = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);
    Ok(Artist {
        id,
        name: name.to_string(),
        created_at: now,
        updated_at: now,
    })
}
