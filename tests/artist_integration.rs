use std::path::PathBuf;

use isolang::Language;
use tyde::{
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

    let artist_name = "Test_Artist_Name";
    let result = repo.add(artist_name, None, None)?;

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

    assert_eq!(artist_name, &name);

    Ok(())
}

#[test]
fn add_sets_kind_and_locale() -> Result<(), Box<dyn std::error::Error>> {
    let lib = setup_library();
    let repo = repository(&lib);

    let artist_name = "Test_Artist_Name";
    let kind = NameKind::Alias;
    let lang = Language::from_639_1("en");
    let result = repo.add(artist_name, Some(kind.clone()), lang.clone())?;

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
    let (row_kind, row_locale): (String, String) = lib.conn.query_row(
        "SELECT kind, locale FROM artist_names WHERE artist_id = ?1",
        [result],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;

    assert_eq!(kind.to_string(), row_kind);

    let lang = lang.unwrap();
    let code = lang.to_639_1().unwrap();

    assert_eq!(code, row_locale);

    Ok(())
}
