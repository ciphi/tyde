use chrono::Utc;
use isolang::Language;
use rusqlite::{Connection, params};
use tracing::{info, instrument};

use anyhow::Result;

use crate::{
    db::types::datetime_stamp::DateTimeStamp, library::artists::model::NameKind, positive,
};

// This struct's only job is to talk to the database for Actors
pub struct ArtistRepository<'a> {
    pub db: &'a Connection,
}

impl<'a> ArtistRepository<'a> {
    /// Add a new artist entry in both artists and artist_names table.
    ///
    /// Returns the row ID of the new entry from the artists table
    #[instrument(name = "add", skip_all)]
    pub fn add(&self, name: &str, kind: Option<NameKind>, locale: Option<Language>) -> Result<i64> {
        let artist_id = self.add_to_artists()?;
        let artist_name_id = self.add_name(artist_id, name, kind, locale)?;
        self.set_primary_name(artist_name_id, artist_id)?;

        println!("Added artist: {}", positive!("{}", name));
        info!(artist_name = name, artist_id, artist_name_id);
        Ok(artist_id)
    }

    /// Add a new entry to the artists table.
    ///
    /// Returns the row ID of the new artist entry.
    fn add_to_artists(&self) -> Result<i64> {
        let now = DateTimeStamp(Utc::now());
        self.db.execute(
            "INSERT INTO artists (created_at, updated_at) VALUES (?1, ?2)",
            params![now.clone(), now],
        )?;
        Ok(self.db.last_insert_rowid())
    }

    /// Add a new entry to the artist_names table.
    ///
    /// Returns the row ID of the entry.
    pub(crate) fn add_name(
        &self,
        artist_id: i64,
        name: &str,
        kind: Option<NameKind>,
        locale: Option<Language>,
    ) -> Result<i64> {
        let now = DateTimeStamp(Utc::now());
        let kind_str: Option<String> = kind.map(|k| k.to_string());
        let locale_str: Option<&str> = locale.and_then(|l| l.to_639_1());

        self.db.execute(
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

        Ok(self.db.last_insert_rowid())
    }

    /// Set the primary artist name.
    fn set_primary_name(&self, name_id: i64, artist_id: i64) -> Result<()> {
        self.db.execute(
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
}
