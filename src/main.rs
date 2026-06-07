mod cli;
mod config;
mod db;
mod logging;
pub mod utils;

use std::process::exit;

use config::Config;
use db::library::Library;
use logging::AppLog;
use tracing::error;

fn main() {
    let _log = AppLog::default().init();

    let cfg = Config::new();
    let data = cfg.load_or_create();

    let Ok(db) = Library::init(data.library.db_path.clone()) else {
        error!("Failed to initialize library");
        exit(1);
    };

    let Ok(cli) = cli::get(&db).map_err(|e| {
        error!("Failed to initialize cli");
        error!("{:?}", e);
        exit(1);
    });

    utils::verbose::set_verbose(cli.verbose);
}
