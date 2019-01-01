use super::APP_INFO;
use app_dirs::{self, AppDataType};
use serde::{self, Deserialize};
use std::{fs::File, io::BufReader};

#[derive(Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub tag_namespace: Option<String>,
    pub images_dir: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        Self::try_load().unwrap_or_else(Self::default)
    }

    fn try_load() -> Option<Self> {
        let config_path = app_dirs::get_app_root(AppDataType::UserConfig, &APP_INFO)
            .ok()?
            .join("config.yml");

        if config_path.exists() {
            let config_file = File::open(&config_path).ok()?;
            let reader = BufReader::new(config_file);

            serde_yaml::from_reader(reader).ok()
        } else {
            None
        }
    }
}
