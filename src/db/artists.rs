use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result, params};

#[derive(Debug)]
struct Artist {
    id: i64,
    /// Canonical name
    name: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
