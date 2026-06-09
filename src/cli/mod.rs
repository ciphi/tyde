pub mod commands;

use clap::{Parser, Subcommand};
use tyde::db::library::Library;

#[derive(Parser)]
#[command(version, about)]
pub(crate) struct Cli {
    /// Enable verbose messages
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Manage artists in the library
    Artist {
        #[command(subcommand)]
        subcmd: commands::artist::ArtistCommands,
    },
}

///Returns a new CLI value.
pub fn get(library: &Library) -> Result<Cli, anyhow::Error> {
    let cli = Cli::parse();
    commands::handle_cli(&library, &cli.command)?;
    Ok(cli)
}
