mod cli;
mod config;
mod logging;
use config::Config;
use logging::AppLog;

pub mod utils;

fn main() {
    let cli = cli::get();
    utils::verbose::set_verbose(cli.verbose);
    let _app_log = AppLog::default().init();
    let cfg = Config::new();
    let data = cfg.load_or_create();
}
