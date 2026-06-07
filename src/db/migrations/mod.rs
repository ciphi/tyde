pub const MIGRATIONS: &[&str] = &[
    include_str!("001_create_artists.sql"),
    include_str!("002_create_artist_names.sql"),
];
