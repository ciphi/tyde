mod cli;
mod config;
mod db;
mod logging;
pub mod utils;

use std::process::exit;

use config::Config;
use db::library::Library;
use logging::AppLog;
use tracing::{error, info};

fn main() {
    let _log = AppLog::default().init();
    if let Err(e) = run() {
        error!("{e}");
    }
}

fn run() -> anyhow::Result<()> {
    let cfg = Config::new();
    let data = cfg.load_or_create();

    let db = Library::init(data.library.db_path.clone()).map_err(|e| {
        negativeln!("Failed to initalize library");
        e
    })?;

    let cli = cli::get(&db).map_err(|e| {
        negativeln!("Failed to initialize CLI");
        e
    })?;

    utils::verbose::set_verbose(cli.verbose);

    Ok(())
}
