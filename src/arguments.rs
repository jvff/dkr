use super::{
    commands::{Commands, RunCommandError},
    config::Config,
};
use std::fmt::{self, Display, Formatter};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Arguments {
    /// Disable configuration volume
    #[structopt(long = "disable-config-volume")]
    disable_config_volume: bool,

    #[structopt(subcommand)]
    command: Commands,
}

impl Arguments {
    pub fn is_config_volume_enabled(&self) -> bool {
        !self.disable_config_volume
    }

    pub fn run_command(self, config: Config) -> Result<(), RunCommandError> {
        self.command.run(config)
    }
}

impl Display for Arguments {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        if self.disable_config_volume {
            write!(formatter, "--disable-config-volume ")?;
        }

        self.command.fmt(formatter)
    }
}
