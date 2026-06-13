pub const MIGRATIONS: &[&str] = &[
    include_str!("001_create_artists.sql"),
    include_str!("002_create_artist_names.sql"),
    include_str!("003_add_normalization_to_artist_names.sql"),
    include_str!("004_set_normalized_column_to_not_null.sql"),
];
