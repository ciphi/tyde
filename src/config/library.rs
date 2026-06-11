use std::path::PathBuf;

use dirs::audio_dir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct LibraryConfig {
    pub directory: PathBuf,
    pub db_path: PathBuf,
}

impl Default for LibraryConfig {
    fn default() -> Self {
        let no_path = "Unable to find a valid music path for the library. \
            Consider setting it in the configuration.";
        let fp = audio_dir().expect(no_path);
        let default_db = fp.join("library.db");
        Self {
            directory: fp,
            db_path: default_db,
        }
    }
}
