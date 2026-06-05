use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct ConfigData {
    pub app: AppConfig,
}

impl Default for ConfigData {
    fn default() -> Self {
        Self {
            app: AppConfig::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct AppConfig {
    pub name: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let app_name = String::from(env!("CARGO_PKG_NAME"));
        Self { name: app_name }
    }
}
