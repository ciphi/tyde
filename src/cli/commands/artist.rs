use clap::{Args, Subcommand};
use tracing::instrument;

use crate::db::{library::Library, models::artists};

#[derive(Subcommand)]
pub(crate) enum ArtistCommands {
    /// Add a new artist
    Add(AddArgs),
}

#[derive(Args)]
pub(crate) struct AddArgs {
    /// Named arg
    #[arg(short, long)]
    name: String,
}

#[instrument(name = "artist", skip_all)]
pub fn handle_command(library: &Library, command: &ArtistCommands) -> rusqlite::Result<()> {
    match command {
        ArtistCommands::Add(args) => artists::insert(&library.conn, &args.name)?,
    };
    Ok(())
}
