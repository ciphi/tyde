use std::{
    fs,
    path::{Path, PathBuf},
};

use rusqlite::{Connection, Error};
use tracing::{info, instrument};

use crate::{db::migrations::MIGRATIONS, library::artists::repository};

#[derive(Debug)]
pub struct Library {
    pub filepath: PathBuf,
    pub conn: Connection,
}

impl Library {
    #[instrument(name = "library_init", skip(filepath))]
    pub fn init(filepath: PathBuf) -> Result<Self, anyhow::Error> {
        Self::ensure_parent_dirs(&filepath)?;

        let conn = Self::connect(&filepath)?;
        Self::migrate(&conn)?;

        Ok(Self { filepath, conn })
    }

    fn ensure_parent_dirs(file_path: &Path) -> std::io::Result<()> {
        if let Some(parent) = file_path.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent)?;
            info!(library_path = %parent.to_string_lossy(), "Created parent directory for database file");
        }
        Ok(())
    }

    fn connect(filepath: &Path) -> Result<Connection, Error> {
        Connection::open(filepath)
    }

    #[instrument(name = "migration", skip(conn))]
    fn migrate(conn: &Connection) -> rusqlite::Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_version (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                version INTEGER NOT NULL
            );",
            [],
        )?;

        let original_version = Self::get_version(conn)?;
        let mut version = original_version;

        while version < MIGRATIONS.len() as i32 {
            conn.execute_batch(MIGRATIONS[version as usize])?;

            version += 1;
            Self::set_version(conn, version)?;

            match version {
                3 => Self::add_normalized_names(conn)?,
                _ => {}
            }
        }

        if original_version < version {
            info!(
                prev = original_version,
                current = version,
                "Library updated"
            )
        }

        Ok(())
    }

    fn add_normalized_names(conn: &Connection) -> rusqlite::Result<()> {
        let mut stmt =
            conn.prepare("SELECT id, name FROM artist_names WHERE normalized_name IS NULL")?;
        let mut rows = stmt.query([])?;

        while let Some(row) = rows.next()? {
            let id: i64 = row.get(0)?;
            let name: String = row.get(1)?;
            let out = repository::ArtistRepository::normalize_name(&name);

            conn.execute(
                "UPDATE artist_names SET normalized_name = ?1 WHERE id = ?2",
                (&out, id),
            )?;
        }

        Ok(())
    }

    fn get_version(conn: &Connection) -> rusqlite::Result<i32> {
        conn.query_row("SELECT version FROM schema_version LIMIT 1", [], |row| {
            row.get(0)
        })
        .or(Ok(0))
    }

    fn set_version(conn: &Connection, v: i32) -> rusqlite::Result<()> {
        conn.execute(
            "INSERT INTO schema_version (id, version)
         VALUES (1, ?1)
         ON CONFLICT(id) DO UPDATE SET version = excluded.version",
            [v],
        )?;
        Ok(())
    }
}
