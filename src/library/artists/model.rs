use crate::db::types::datetime_stamp::DateTimeStamp;
use strum::{Display, EnumString};

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

#[derive(Debug, Clone, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub enum NameKind {
    Legal,
    Stage,
    Alias,
}

impl NameKind {
    pub fn as_title(&self) -> &'static str {
        match self {
            NameKind::Alias => "Alias",
            NameKind::Legal => "Legal",
            NameKind::Stage => "Stage",
        }
    }
}

// pub(crate) fn lookup_names(conn: &Connection, artist: &[&str]) -> Result<Vec(ArtistName)> {
//     let placeholders = std::iter::repeat("?")
//         .take(artist.len())
//         .collect::<Vec<_>>()
//         .join(",");
//
//     let query =
//         format!("SELECT ( id, artist_id, name, kind, locale, created_at, updated_at) WHERE name");
//
//     Ok(())
// }

// pub(crate) fn print_entries(conn: &Connection, artists: &[String]) {
//     // lookup for each arist
//     for artist in artists {
//         println!("{:?}", artist);
//     }
// }
