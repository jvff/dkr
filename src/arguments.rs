use super::{
    commands::{Build, Clean, New, Run, RunBuildError, RunCleanError, RunNewError, RunRunError},
    config::Config,
};
use failure::Fail;
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Arguments {
    #[structopt(name = "build")]
    Build(Build),

    #[structopt(name = "clean")]
    Clean(Clean),

    #[structopt(name = "new")]
    New(New),

    #[structopt(name = "run")]
    Run(Run),
}

#[derive(Debug, Fail)]
pub enum RunCommandError {
    #[fail(display = "Failed to build image")]
    Build(#[cause] RunBuildError),

    #[fail(display = "Failed to remove stale images")]
    Clean(#[cause] RunCleanError),

    #[fail(display = "Failed to create new project")]
    New(#[cause] RunNewError),

    #[fail(display = "Failed to run project environment")]
    Run(#[cause] RunRunError),
}

impl Arguments {
    pub fn run_command(self, config: Config) -> Result<(), RunCommandError> {
        match self {
            Arguments::Build(build) => build.run(config).map_err(RunCommandError::Build),
            Arguments::Clean(clean) => clean.run().map_err(RunCommandError::Clean),
            Arguments::New(new) => new.run().map_err(RunCommandError::New),
            Arguments::Run(run) => run.run().map_err(RunCommandError::Run),
        }
    }
}
