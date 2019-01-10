use super::super::docker_environment::{
    DockerEnvironment, LoadEnvironmentError, RunEnvironmentError,
};
use failure::Fail;
use std::fmt::{self, Display, Formatter};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Run {
    environment: String,
    project: String,
}

#[derive(Debug, Fail)]
pub enum RunRunError {
    #[fail(display = "Failed to load environment configuration")]
    LoadEnvironmentError(#[cause] LoadEnvironmentError),

    #[fail(display = "Failed to run environment configuration")]
    RunError(#[cause] RunEnvironmentError),
}

impl Run {
    pub fn run(self) -> Result<(), RunRunError> {
        let environment = DockerEnvironment::load(&self.environment)
            .map_err(RunRunError::LoadEnvironmentError)?;

        environment.run(self.project).map_err(RunRunError::RunError)
    }
}

impl Display for Run {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "run {} {}", self.environment, self.project)
    }
}
