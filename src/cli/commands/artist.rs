use clap::{Args, Subcommand};
use tracing::instrument;

use crate::db::artists;
use crate::db::library::Library;

#[derive(Subcommand)]
pub enum ArtistCommands {
    /// Add a new artist
    Add(AddArgs),
}

#[derive(Args)]
struct AddArgs {
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
