use anyhow::{Result, bail};
use chrono::Utc;
use clap::ValueEnum;
use isolang::Language;
use rusqlite::{Connection, params};
use strum::{Display, EnumString};
use tracing::{info, instrument};

use crate::{db::types::datetime_stamp::DateTimeStamp, positive};

#[derive(Debug)]
pub(crate) struct Artist {
    pub id: i64,
    /// Primary name from ArtistName table.
    pub primary_name_id: i64,
    pub created_at: DateTimeStamp,
    pub updated_at: DateTimeStamp,
}

#[derive(Debug)]
pub(crate) struct ArtistName {
    pub id: i64,
    /// References ID from Artist table.
    pub artist_id: String,
    /// Localized name.
    pub name: String,
    /// 2 char
    pub locale: String,
    pub kind: NameKind,
    pub created_at: DateTimeStamp,
    pub updated_at: DateTimeStamp,
}

#[derive(Debug, Clone, Display, EnumString, ValueEnum)]
#[strum(serialize_all = "lowercase")]
pub enum NameKind {
    Legal,
    Stage,
    Alias,
}

/// Add a new artist entry in both artists and artist_names table.
///
/// Returns the row ID of the new entry from the artists table
#[instrument(name = "add", skip_all)]
pub fn add(
    conn: &Connection,
    name: &str,
    kind: Option<NameKind>,
    locale: Option<Language>,
) -> Result<i64> {
    let artist_id = add_artist(conn)?;
    let artist_name_id = add_artist_name(conn, artist_id, name, kind, locale)?;
    set_primary_name(conn, artist_name_id, artist_id)?;

    println!("Added artist: {}", positive!("{}", name));
    info!(artist_name = name, artist_id, artist_name_id);
    Ok(artist_id)
}

/// Add a new entry to the artists table.
///
/// Returns the row ID of the new artist entry.
fn add_artist(conn: &Connection) -> Result<i64> {
    let now = DateTimeStamp(Utc::now());
    conn.execute(
        "INSERT INTO artists (created_at, updated_at) VALUES (?1, ?2)",
        params![now.clone(), now],
    )?;
    Ok(conn.last_insert_rowid())
}

/// Add a new entry to the artist_names table.
///
/// Returns the row ID of the entry.
fn add_artist_name(
    conn: &Connection,
    artist_id: i64,
    name: &str,
    kind: Option<NameKind>,
    locale: Option<Language>,
) -> Result<i64> {
    let now = DateTimeStamp(Utc::now());
    let kind_str: Option<String> = kind.map(|k| k.to_string());
    let locale_str: Option<&str> = locale.and_then(|l| l.to_639_1());

    conn.execute(
        r#"
        INSERT INTO artist_names (
            artist_id, 
            name, 
            kind, 
            locale, 
            created_at, 
            updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)"#,
        params![&artist_id, name, kind_str, locale_str, &now, &now],
    )?;

    Ok(conn.last_insert_rowid())
}

/// Set the primary artist name.
fn set_primary_name(conn: &Connection, name_id: i64, artist_id: i64) -> Result<()> {
    conn.execute(
        r#"
        UPDATE artists 
        SET primary_name_id = (?1)
        WHERE id = (?2)
        "#,
        params![name_id, artist_id],
    )?;
    info!(primary_name_id = name_id, artist_id, "Primary name set");
    Ok(())
}
