mod shared_volume;

use self::shared_volume::SharedVolume;
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
    shared_volumes: Vec<SharedVolume>,
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

#[derive(Debug, Fail)]
#[fail(display = "Failed to run environment")]
pub struct RunEnvironmentError(#[cause] io::Error);

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
        let mut arguments = vec!["run".to_owned(), "--rm".to_owned()];

        arguments.push("-v".to_owned());
        arguments.push(format!("{}:/project", project));

        arguments.push("-e".to_owned());
        arguments.push(format!("NAME={}", project));

        for shared_volume in &self.shared_volumes {
            arguments.push("-v".to_owned());
            arguments.push(shared_volume.volume_argument());
        }

        arguments.push(self.image.clone());

        arguments.push("sh".to_owned());
        arguments.push("-c".to_owned());
        arguments.push(self.new.clone());

        cmd("docker", &arguments)
            .run()
            .map_err(InitializeEnvironmentError)?;

        Ok(())
    }

    pub fn run(&self, project: String) -> Result<(), RunEnvironmentError> {
        let mut arguments = vec!["run".to_owned(), "--rm".to_owned(), "-it".to_owned()];

        arguments.push("-v".to_owned());
        arguments.push(format!("{}:/project", project));

        for shared_volume in &self.shared_volumes {
            arguments.push("-v".to_owned());
            arguments.push(shared_volume.volume_argument());
        }

        arguments.push(self.image.clone());

        cmd("docker", &arguments)
            .run()
            .map_err(RunEnvironmentError)?;

        Ok(())
    }
}
