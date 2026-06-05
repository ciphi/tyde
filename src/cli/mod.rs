use clap::{Args, Parser, Subcommand};
use tracing::instrument;

#[derive(Parser)]
#[command(version, about)]
pub(crate) struct Cli {
    /// Enable verbose messages
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Args)]
struct ExampleArgs {
    /// Named arg
    #[arg(short, long)]
    name: Option<String>,

    /// Chained args
    #[arg(short, long)]
    chain: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// A subcommand you can call
    Example(ExampleArgs),
}

fn example_command(args: &ExampleArgs) {
    println!("Subcommand called");

    if let Some(name) = &args.name {
        println!("With argument: {}", name);
    }

    if !&args.chain.is_empty() {
        let mut values = String::from("With chained argument: ");
        values.push_str(&args.chain.join(", "));
        println!("{}", values);
    }
}

///Returns a new CLI value.
#[instrument(name = "CLI")]
pub fn get() -> Cli {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Example(args) => example_command(&args),
    }
    cli
}
