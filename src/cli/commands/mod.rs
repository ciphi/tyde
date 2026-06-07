pub(crate) mod artist;

use crate::{cli::Commands, db::library::Library};
use rusqlite::Connection;

/// Level 1 Router: Directs the global command to the correct module
pub fn handle_cli(library: &Library, global_command: &Commands) -> rusqlite::Result<()> {
    match global_command {
        Commands::Artist { subcmd } => {
            // Forward only the specific Actor subcommands down
            artist::handle_command(library, subcmd)?;
        }
    }
    Ok(())
}
