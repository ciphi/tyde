use anyhow::Result;
use clap::{Args, Subcommand};
use inquire::Confirm;
use isolang::Language;
use std::{fmt, str::FromStr};
use tracing::instrument;

use crate::{
    db::library::Library,
    library::artists::{
        model::{ArtistName, NameKind},
        repository::ArtistRepository,
    },
    negativeln, noticeln,
};

#[derive(Subcommand)]
pub(crate) enum ArtistCommands {
    #[command(
        about = "Add a new artist",
        long_about = "\
Each name and variant argument uses a semicolon-delimited format.
Types: legal, stage, alias
Language must be 639_1 compliant.

Example: \"John Doe;type=legal;lang=en\"
"
    )]
    Add(AddArgs),
}

#[derive(Args)]
pub(crate) struct AddArgs {
    /// Artist name
    #[arg(short, long, value_parser = validate_name, help = "Primary artist name with optional metadata" )]
    name: NameRecord,

    #[arg(short, long, value_parser = validate_name_variant)]
    variant: Vec<NameVariantRecord>,
}

#[derive(Debug, Clone)]
pub struct NameRecord {
    pub name: String,

    /// Locale code of the artist name in ISO-639-1 format
    pub locale: Option<Language>,

    /// Represents the type of artist name
    pub name_type: Option<NameKind>,
}

fn format_record(record: &NameRecord) -> String {
    let mut output = format!("{}", record.name);
    let meta: Vec<String> = vec![
        record.name_type.as_ref().map(|t| t.as_title().to_string()),
        record.locale.as_ref().map(|l| l.to_string()),
    ]
    .into_iter()
    .filter_map(|x| x)
    .collect();

    if !meta.is_empty() {
        output.push_str(&format!(" ({})", meta.join(", ")));
    }

    output
}

impl fmt::Display for NameRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_record(self))?;
        Ok(())
    }
}

impl NameRecord {
    pub fn new(name: String, locale: Option<Language>, name_type: Option<NameKind>) -> Self {
        Self {
            name,
            locale,
            name_type,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NameVariantRecord {
    pub(crate) record: NameRecord,
}

impl fmt::Display for NameVariantRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_record(&self.record))?;
        Ok(())
    }
}

impl NameVariantRecord {
    pub fn new(record: NameRecord) -> Self {
        Self { record }
    }
}

#[derive(Args)]
pub(crate) struct ShowArgs {
    /// Artist name
    #[arg(short, long, required = true)]
    name: Vec<String>,
}

fn validate_name(s: &str) -> Result<NameRecord, String> {
    let mut values = s.split(";");
    let mut lang: Option<Language> = None;
    let mut kind: Option<NameKind> = None;

    let name = values.next().unwrap();
    for key in values {
        if let Some((key, value)) = key.split_once('=') {
            match key {
                "locale" => {
                    lang = Some(Language::from_639_1(value).ok_or(String::from(
                        "Ensure the value provided is ISO-639-1 compliant.",
                    ))?);
                }
                "type" => {
                    kind = NameKind::from_str(value).ok();
                }
                _ => {}
            }
        }
    }

    Ok(NameRecord {
        name: name.to_string(),
        locale: lang,
        name_type: kind,
    })
}

fn validate_name_variant(s: &str) -> Result<NameVariantRecord, String> {
    let test = validate_name(&s)?;
    Ok(NameVariantRecord { record: test })
}

#[instrument(name = "artist", skip_all)]
pub(crate) fn handle_command(library: &Library, command: &ArtistCommands) -> Result<()> {
    let repo = ArtistRepository { db: &library.conn };
    match command {
        ArtistCommands::Add(args) => handle_add_command(&repo, &args)?,
        }
    };
    Ok(())
}

pub(crate) fn handle_add_command(repository: &ArtistRepository, args: &AddArgs) -> Result<()> {
    println!("Artist: {}", args.name);
    if !args.variant.is_empty() {
        let mut variants_display = String::from("Variants:");
        for variant in &args.variant {
            variants_display.push_str(&format!("\n  {}", variant));
        }
        println!("{}", variants_display);
    }

    let result = Confirm::new("Add entry?").with_default(false).prompt();
    match result {
        Ok(true) => {
            let _ = repository.add(&args.name, &args.variant)?;
        }
        Ok(false) => {
            println!("Cancelled")
        }
        Err(_) => println!("Error or aborted"),
    }

    Ok(())
}
