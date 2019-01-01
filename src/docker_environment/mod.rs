mod shared_volume;

use self::shared_volume::SharedVolume;
use super::{docker, APP_INFO};
use app_dirs::{self, AppDataType, AppDirsError};
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
    project_path: Option<String>,
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
        let mut command = docker::run(&self.image);

        command
            .temporary()
            .volume(&project, self.project_path())
            .env("NAME", project);

        for shared_volume in &self.shared_volumes {
            command.volume(shared_volume.name(), shared_volume.at());
        }

        command
            .run_shell_command(&self.new)
            .map_err(InitializeEnvironmentError)
    }

    pub fn run(&self, project: String) -> Result<(), RunEnvironmentError> {
        let mut command = docker::run(&self.image);

        command
            .temporary()
            .interactive()
            .volume(project, self.project_path());

        for shared_volume in &self.shared_volumes {
            command.volume(shared_volume.name(), shared_volume.at());
        }

        command.run().map_err(RunEnvironmentError)
    }

    fn project_path(&self) -> &str {
        match self.project_path.as_ref() {
            Some(path) => path,
            None => "/project",
        }
    }
}
