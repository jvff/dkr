use failure::Fail;
use structopt::StructOpt;
use super::{config::Config, commands::{Build, RunBuildError, Clean, RunCleanError}};

#[derive(StructOpt)]
pub enum Arguments {
    #[structopt(name = "build")]
    Build(Build),

    #[structopt(name = "clean")]
    Clean(Clean),
}

#[derive(Debug, Fail)]
pub enum RunCommandError {
    #[fail(display = "Failed to build image")]
    Build(#[cause] RunBuildError),

    #[fail(display = "Failed to remove stale images")]
    Clean(#[cause] RunCleanError),
}

impl Arguments {
    pub fn run_command(self, config: Config) -> Result<(), RunCommandError> {
        match self {
            Arguments::Build(build) => build.run(config).map_err(RunCommandError::Build),
            Arguments::Clean(clean) => clean.run().map_err(RunCommandError::Clean),
        }
    }
}
