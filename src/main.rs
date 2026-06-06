mod cli;
mod config;
mod db;
mod logging;

use config::Config;
use db::library::Library;
use logging::AppLog;

pub mod utils;

fn main() {
    let _log = AppLog::default().init();

    let cfg = Config::new();
    let data = cfg.load_or_create();

    let _db = Library::init(data.library.db_path.clone());

    let cli = cli::get();
    utils::verbose::set_verbose(cli.verbose);
}
