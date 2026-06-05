use std::{fs, io::Read, path::PathBuf};

use tracing::trace;

pub mod data;
use data::ConfigData;

pub(crate) struct Config {
    pub directory: PathBuf,
    pub filename: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let mut base_dir = dirs::config_dir().expect("Unable to find config directory.");

        let app_name = env!("CARGO_PKG_NAME");
        base_dir.push(app_name);

        let filepath = base_dir.join("config.toml");

        trace!("Config path set to: {}", filepath.to_string_lossy());

        Self {
            directory: base_dir,
            filename: filepath,
        }
    }

    pub(crate) fn ensure_exists(&self) {
        fs::create_dir_all(&self.directory).expect("Failed to create config directory");

        if !self.filename.exists() {
            let default = ConfigData::default();
            self.write(&default);
        }
    }

    pub(crate) fn load(&self) -> ConfigData {
        let mut file = fs::File::open(&self.filename).expect("Failed to open config file");

        let mut buffer = String::new();

        file.read_to_string(&mut buffer)
            .expect("Failed to read config file");

        toml::from_str(&buffer).expect("Failed to parse config file")
    }

    pub(crate) fn load_or_create(&self) -> ConfigData {
        self.ensure_exists();
        let data = self.load();
        data
    }

    fn write(&self, data: &ConfigData) {
        let toml = toml::to_string(data).expect("Failed to serialize config");

        fs::write(&self.filename, toml).expect("Failed to write config");
    }
}
