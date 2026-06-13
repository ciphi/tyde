use std::path::PathBuf;

use isolang::Language;
use tyde::{
    cli::commands::artist::{NameRecord, NameVariantRecord},
    db::library::Library,
    library::artists::{model::NameKind, repository::ArtistRepository},
};

fn setup_library() -> Library {
    Library::init(PathBuf::from(":memory:")).unwrap()
}

fn repository(library: &Library) -> ArtistRepository<'_> {
    ArtistRepository { db: &library.conn }
}

#[test]
fn add_creates_name_and_sets_primary() -> Result<(), Box<dyn std::error::Error>> {
    let lib = setup_library();
    let repo = repository(&lib);

    let record = NameRecord::new("Test Artist".into(), None, None);
    let variant: Vec<NameVariantRecord> = Vec::new();
    let result = repo.add(&record, &variant)?;

    // Assert the row was created in artists table.
    assert_eq!(result, 1);

    // Get the primary ID from the new row
    let primary_name_id: i64 = lib.conn.query_row(
        "SELECT primary_name_id FROM artists WHERE id = ? ",
        [result],
        |row| row.get(0),
    )?;

    assert_eq!(primary_name_id, result);

    // Get the name based on the primary ID from artist_names table
    let name: String = lib.conn.query_row(
        "SELECT name FROM artist_names WHERE artist_id = ? ",
        [result],
        |row| row.get(0),
    )?;

    assert_eq!(&record.name, &name);

    Ok(())
}

#[test]
fn add_sets_kind_and_lang() -> Result<(), Box<dyn std::error::Error>> {
    let lib = setup_library();
    let repo = repository(&lib);

    let lang = Language::from_639_1("en");
    let kind = Some(NameKind::Alias);
    let record = NameRecord::new("Test Artist Name".into(), lang, kind);

    let variant: Vec<NameVariantRecord> = Vec::new();
    let result = repo.add(&record, &variant)?;

    // Assert the row was created in artists table.
    assert_eq!(result, 1);

    // Get the primary ID from the new row
    let primary_name_id: i64 = lib.conn.query_row(
        "SELECT primary_name_id FROM artists WHERE id = ? ",
        [result],
        |row| row.get(0),
    )?;

    assert_eq!(primary_name_id, result);

    // Get the name based on the primary ID from artist_names table
    let (row_kind, row_lang): (String, String) = lib.conn.query_row(
        "SELECT kind, lang FROM artist_names WHERE artist_id = ?1",
        [result],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;

    assert_eq!(record.name_type.unwrap().to_string(), row_kind);
    assert_eq!(record.lang.unwrap().to_639_1().unwrap(), row_lang);

    Ok(())
}

#[test]
fn add_apply_all_variants() -> Result<(), Box<dyn std::error::Error>> {
    let lib = setup_library();
    let repo = repository(&lib);

    let lang = Language::from_639_1("en");
    let kind = Some(NameKind::Alias);
    let record = NameRecord::new("Test Artist Name".into(), lang, kind);
    let mut variants: Vec<NameVariantRecord> = Vec::new();

    for i in 0..5 {
        let variant_name = format!("Name Variant {i}");
        let variant_lang = Language::from_639_1("en");
        let variant_kind = Some(NameKind::Alias);
        let variant_record = NameRecord::new(variant_name, variant_lang, variant_kind);
        variants.push(NameVariantRecord::new(variant_record));
    }

    let result = repo.add(&record, &variants)?;

    // Assert the row was created in artists table.
    assert_eq!(result, 1);

    // Get the primary ID from the new row
    let primary_name_id: i64 = lib.conn.query_row(
        "SELECT primary_name_id FROM artists WHERE id = ? ",
        [result],
        |row| row.get(0),
    )?;

    assert_eq!(primary_name_id, result);

    for i in 0..variants.len() {
        let variant_name = format!("Name Variant {i}");
        let (artist_id, row_kind, row_lang): (i64, String, String) = lib.conn.query_row(
            "SELECT artist_id, kind, lang FROM artist_names WHERE name = ?1",
            [variant_name],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;

        assert_eq!(result, artist_id);
        assert_eq!(record.name_type.unwrap().to_string(), row_kind);
        assert_eq!(record.lang.unwrap().to_639_1().unwrap(), row_lang);
    }

    Ok(())
}
