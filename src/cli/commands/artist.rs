use anyhow::Result;
use clap::{Args, Subcommand};
use isolang::Language;
use tracing::instrument;
use tyde::{
    db::library::Library,
    library::artists::{model::NameKind, repository::ArtistRepository},
};

#[derive(Subcommand)]
pub(crate) enum ArtistCommands {
    /// Add a new artist
    Add(AddArgs),
}

#[derive(Args)]
pub(crate) struct AddArgs {
    /// Artist name
    #[arg(short, long)]
    name: String,

    /// Locale code of the artist name in ISO-639-1 format
    #[arg(short, long, value_parser = validate_locale)]
    locale: Option<Language>,

    /// Represents the type of artist name
    #[arg(short = 't', long)]
    name_type: Option<NameKind>,
}

fn validate_locale(s: &str) -> Result<Language, String> {
    Language::from_639_1(s).ok_or(String::from(
        "Ensure the value provided is ISO-639-1 compliant.",
    ))
}

#[instrument(name = "artist", skip_all)]
pub fn handle_command(library: &Library, command: &ArtistCommands) -> Result<()> {
    let repo = ArtistRepository { db: &library.conn };
    match command {
        ArtistCommands::Add(args) => {
            repo.add(&args.name, args.name_type.clone(), args.locale.clone())?;
        }
    };
    Ok(())
}
