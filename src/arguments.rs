use failure::Fail;
use structopt::StructOpt;
use super::{config::Config, commands::{Build, RunBuildError}};

#[derive(StructOpt)]
pub enum Arguments {
    #[structopt(name = "build")]
    Build(Build),
}

#[derive(Debug, Fail)]
pub enum RunCommandError {
    #[fail(display = "Failed to build image")]
    Build(#[cause] RunBuildError),
}

impl Arguments {
    pub fn run_command(self, config: Config) -> Result<(), RunCommandError> {
        match self {
            Arguments::Build(build) => build.run(config).map_err(RunCommandError::Build),
        }
    }
}
