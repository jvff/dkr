use super::{
    commands::{Commands, RunCommandError},
    config::Config,
};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Arguments {
    #[structopt(subcommand)]
    command: Commands,
}

impl Arguments {
    pub fn run_command(self, config: Config) -> Result<(), RunCommandError> {
        self.command.run(config)
    }
}
