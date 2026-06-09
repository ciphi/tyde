pub(crate) mod artist;

use anyhow::Result;
use tyde::db::library::Library;

use crate::cli::Commands;

/// Level 1 Router: Directs the global command to the correct module
pub fn handle_cli(library: &Library, global_command: &Commands) -> Result<()> {
    match global_command {
        Commands::Artist { subcmd } => {
            // Forward only the specific Actor subcommands down
            artist::handle_command(library, subcmd)?;
        }
    }
    Ok(())
}
