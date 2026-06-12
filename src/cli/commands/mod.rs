pub mod artist;

use anyhow::Result;

use crate::{cli::Commands, db::library::Library};

/// Level 1 Router: Directs the global command to the correct module
pub(crate) fn handle_cli(library: &Library, global_command: &Commands) -> Result<()> {
    match global_command {
        Commands::Artist { subcmd } => {
            // Forward only the specific Actor subcommands down
            artist::handle_command(library, subcmd)?;
        }
    }
    Ok(())
}
