use std::str::FromStr;

use chrono::Utc;
use isolang::Language;
use rusqlite::{Connection, params};
use tracing::{info, instrument};
use unicode_normalization::UnicodeNormalization;

use anyhow::Result;

use crate::{
    cli::commands::artist::{NameRecord, NameVariantRecord},
    db::types::datetime_stamp::DateTimeStamp,
    library::artists::model::{ArtistName, NameKind},
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
        let artist_name_id = self.add_name(artist_id, &name.name, &name.name_type, &name.locale)?;

        self.set_primary_name(artist_name_id, artist_id)?;

        for variant in variants {
            self.add_name(
                artist_id,
                &variant.record.name,
                &variant.record.name_type,
                &variant.record.locale,
            )?;
        }

        let variant_label = match variants.len() {
            2.. => &format!("(with {} variants)", variants.len()),
            1.. => "(with 1 variant)",
            _ => "",
        };

        println!(
            "Added artist: {} {}",
            positive!("{}", name.name),
            variant_label
        );

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
            normalized_name,
            kind, 
            locale, 
            created_at, 
            updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"#,
            params![
                &artist_id,
                name,
                Self::normalize_name(name),
                kind_str,
                locale_str,
                &now,
                &now
            ],
        )?;

        info!(artist_name = name, artist_id);

        Ok(self.db.last_insert_rowid())
    }

    /// Get all rows that are marked as a primary name.
    fn extract_primary_names(&self, name: &str) -> Result<Vec<ArtistName>> {
        let mut stmt = self.db.prepare(
            r#"
            SELECT *
            FROM artist_names
            WHERE id IN (
                SELECT primary_name_id FROM artists
            )
            "#,
        )?;

        let artists: Vec<ArtistName> = stmt
            .query_map(
                [],
                |row| -> std::prelude::v1::Result<ArtistName, rusqlite::Error> {
                    Ok(ArtistName {
                        id: row.get(0)?,
                        artist_id: row.get(1)?,
                        name: row.get(2)?,
                        normalized_name: row.get(3)?,
                        kind: row
                            .get::<_, Option<String>>(4)?
                            .map(|s| NameKind::from_str(&s))
                            .transpose()
                            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                        locale: row.get(5)?,
                        created_at: row.get(6)?,
                        updated_at: row.get(7)?,
                    })
                },
            )?
            .collect::<Result<_, _>>()?;
        Ok(artists)
    }

    pub(crate) fn normalize_name(s: &str) -> String {
        s.nfkc()
            .flat_map(|c| c.to_lowercase())
            .filter(|c| c.is_alphanumeric())
            .collect()
    }

    /// Check if a name exists as a primary name in the artists table.
    ///
    /// # Returns
    ///
    /// Row(s) containing the primary name if any.
    pub(crate) fn find_artist_by_primary_name(&self, name: &str) -> Result<Vec<ArtistName>> {
        let names = self.extract_primary_names(name)?;
        Ok(names
            .into_iter()
            .filter(|primary_name| {
                primary_name
                    .name
                    .nfkc()
                    .flat_map(|c| c.to_lowercase())
                    .eq(name.nfkc().flat_map(|c| c.to_lowercase()))
            })
            .collect())
    }

    pub(crate) fn find_artist_by_name(&self, name: &str) -> Result<Vec<ArtistName>> {
        let mut stmt = self
            .db
            .prepare("SELECT * FROM artist_names WHERE name = ?;")?;

        let artists: Vec<ArtistName> = stmt
            .query_map([&name], |row| {
                Ok(ArtistName {
                    id: row.get(0)?,
                    artist_id: row.get(1)?,
                    name: row.get(2)?,
                    normalized_name: row.get(3)?,
                    kind: row
                        .get::<_, Option<String>>(4)?
                        .map(|s| NameKind::from_str(&s))
                        .transpose()
                        .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?,
                    locale: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            })?
            .collect::<Result<_, _>>()?;
        Ok(artists)
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
