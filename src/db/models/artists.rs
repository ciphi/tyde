use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result, params};
use tracing::{info, instrument};

use crate::{db::types::datetime_stamp::DateTimeStamp, positive};

#[derive(Debug)]
pub(crate) struct Artist {
    pub id: i64,
    /// Canonical name
    pub name: String,
    pub created_at: DateTimeStamp,
    pub updated_at: DateTimeStamp,
}

#[instrument(name = "add", skip_all)]
pub fn insert(conn: &Connection, name: &str) -> Result<Artist> {
    let now = DateTimeStamp(Utc::now());

    conn.execute(
        "INSERT INTO artists (name, created_at, updated_at) VALUES (?1, ?2, ?3)",
        params![&name, &now, &now],
    )?;

    let id = conn.last_insert_rowid();

    println!("Added artist: {}", positive!("{}", name));
    info!(artist_name = name, created_at = now.to_string());

    Ok(Artist {
        id,
        name: name.to_string(),
        created_at: now.clone(),
        updated_at: now,
    })
}
