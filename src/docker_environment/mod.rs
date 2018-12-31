use super::APP_INFO;
use app_dirs::{self, AppDataType, AppDirsError};
use duct::cmd;
use failure::Fail;
use serde::Deserialize;
use std::{
    fs::File,
    io::{self, BufReader},
};

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DockerEnvironment {
    image: String,
    new: String,
}

#[derive(Debug, Fail)]
pub enum LoadEnvironmentError {
    #[fail(display = "Failed to retrieve configuration directory")]
    ConfigDirError(#[cause] AppDirsError),

    #[fail(display = "Failed to parse environment file")]
    DeserializationError(#[cause] serde_yaml::Error),

    #[fail(display = "Failed to open environment file: {}", _0)]
    EnvFileError(String, #[cause] io::Error),
}

#[derive(Debug, Fail)]
#[fail(display = "Failed to run initialization command")]
pub struct InitializeEnvironmentError(#[cause] io::Error);

impl DockerEnvironment {
    pub fn load(name: &str) -> Result<Self, LoadEnvironmentError> {
        let path = app_dirs::get_app_root(AppDataType::UserConfig, &APP_INFO)
            .map_err(LoadEnvironmentError::ConfigDirError)?
            .join("environments")
            .join(format!("{}.yml", name));

        let file = File::open(&path).map_err(|cause| {
            LoadEnvironmentError::EnvFileError(path.display().to_string(), cause)
        })?;

        let reader = BufReader::new(file);

        serde_yaml::from_reader(reader).map_err(LoadEnvironmentError::DeserializationError)
    }

    pub fn initialize(&self, project: String) -> Result<(), InitializeEnvironmentError> {
        cmd!(
            "docker",
            "run",
            "--rm",
            "-v",
            format!("{}:/project", project),
            "-e",
            format!("NAME={}", project),
            &self.image,
            "sh",
            "-c",
            &self.new
        )
        .run()
        .map_err(InitializeEnvironmentError)?;

        Ok(())
    }
}
