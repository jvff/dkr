use super::super::docker_environment::{
    DockerEnvironment, InitializeEnvironmentError, LoadEnvironmentError,
};
use duct::cmd;
use failure::Fail;
use std::io;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct New {
    environment: String,
    name: String,
}

#[derive(Debug, Fail)]
pub enum RunNewError {
    #[fail(display = "Failed to create docker volume")]
    CreateVolumeError(#[cause] io::Error),

    #[fail(display = "Failed to initialization environment")]
    InitializeEnvironmentError(#[cause] InitializeEnvironmentError),

    #[fail(display = "Failed to load environment configuration")]
    LoadEnvironmentError(#[cause] LoadEnvironmentError),
}

impl New {
    pub fn run(self) -> Result<(), RunNewError> {
        let environment = DockerEnvironment::load(&self.environment)
            .map_err(RunNewError::LoadEnvironmentError)?;

        cmd!("docker", "volume", "create", "--name", &self.name)
            .run()
            .map_err(RunNewError::CreateVolumeError)?;

        environment
            .initialize(self.name)
            .map_err(RunNewError::InitializeEnvironmentError)?;

        Ok(())
    }
}
