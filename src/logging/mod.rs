use dirs;
use std::{fs, path::PathBuf};
use tracing::trace;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_rolling_file::*;
use tracing_subscriber::filter::EnvFilter;

pub struct AppLog {
    pub directory: PathBuf,
    pub filepath: PathBuf,
}

pub struct ActiveLog {
    _guard: WorkerGuard,
}

impl Default for AppLog {
    fn default() -> Self {
        let base_dir = dirs::state_dir()
            .or_else(|| dirs::data_local_dir())
            .expect("no suitable directory");

        let app_name = env!("CARGO_PKG_NAME");

        let directory = base_dir.join(app_name).join("logs");
        let filepath = directory.join(format!("{app_name}.log"));

        Self {
            directory,
            filepath,
        }
    }
}

impl AppLog {
    pub fn init(&self) -> ActiveLog {
        use tracing_subscriber::filter::LevelFilter;
        use tracing_subscriber::{fmt, prelude::*};

        let env_filter =
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("error"));

        let console_layer = fmt::layer()
            .without_time()
            .with_target(false)
            .with_filter(env_filter);

        fs::create_dir_all(&self.directory).expect("Failed to create log directory.");

        let file_builder = RollingFileAppenderBase::builder();
        let appender = file_builder
            .filename(
                self.filepath
                    .clone()
                    .into_os_string()
                    .into_string()
                    .expect("Path contains invalid UTF-8 characters"),
            )
            .max_filecount(3)
            .condition_max_file_size(10000)
            .build()
            .unwrap();

        let (writer, guard) = appender.get_non_blocking_appender();
        let file_layer = fmt::layer()
            .with_writer(writer)
            .with_ansi(false)
            .with_target(false)
            .with_filter(LevelFilter::DEBUG);

        let init_result = tracing_subscriber::registry()
            .with(console_layer)
            .with(file_layer)
            .try_init();

        match init_result {
            Ok(_) => trace!("File log is set to {}", self.filepath.display()),
            Err(e) => panic!(
                "CRITICAL: Something else already registered a logger: {:?}",
                e
            ),
        }

        ActiveLog { _guard: guard }
    }
}
