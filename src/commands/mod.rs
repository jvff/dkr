mod build;
mod clean;
mod new;
mod run;

pub use self::{
    build::{Build, RunBuildError},
    clean::{Clean, RunCleanError},
    new::{New, RunNewError},
    run::{Run, RunRunError},
};
use super::config::Config;
use failure::Fail;
use std::fmt::{self, Display, Formatter};
use structopt::StructOpt;

#[derive(StructOpt)]
pub enum Commands {
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

impl Commands {
    pub fn run(self, config: Config) -> Result<(), RunCommandError> {
        match self {
            Commands::Build(build) => build.run(config).map_err(RunCommandError::Build),
            Commands::Clean(clean) => clean.run().map_err(RunCommandError::Clean),
            Commands::New(new) => new.run().map_err(RunCommandError::New),
            Commands::Run(run) => run.run().map_err(RunCommandError::Run),
        }
    }
}

impl Display for Commands {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Commands::Build(build) => build.fmt(formatter),
            Commands::Clean(clean) => clean.fmt(formatter),
            Commands::New(new) => new.fmt(formatter),
            Commands::Run(run) => run.fmt(formatter),
        }
    }
}
