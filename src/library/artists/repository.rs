use chrono::Utc;
use isolang::Language;
use rusqlite::{Connection, params};
use tracing::{info, instrument};

use anyhow::Result;

use crate::{
    cli::commands::artist::{NameRecord, NameVariantRecord},
    db::types::datetime_stamp::DateTimeStamp,
    library::artists::model::NameKind,
    positive,
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
    pub fn add(&self, name: &NameRecord, variants: &Vec<NameVariantRecord>) -> Result<i64> {
        let artist_id = self.add_to_artists()?;
        // Add primary first
        let artist_name_id = self.add_name(artist_id, &name.name, &name.name_type, &name.locale)?;
        self.set_primary_name(artist_name_id, artist_id)?;
        // Add variants
        for variant in variants {
            self.add_name(
                artist_id,
                &variant.record.name,
                &variant.record.name_type,
                &variant.record.locale,
            )?;
        }

        println!("Added artist: {}", positive!("{}", name));
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
        kind: &Option<NameKind>,
        locale: &Option<Language>,
    ) -> Result<i64> {
        let now = DateTimeStamp(Utc::now());
        let kind_str: Option<String> = kind.clone().map(|k| k.to_string());
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

        info!(artist_name = name, artist_id);

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
