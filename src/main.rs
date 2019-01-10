mod arguments;
mod commands;
mod config;
mod docker;
mod docker_environment;
mod docker_image;
mod dockerfile;

use std::io;
use self::{arguments::Arguments, commands::RunCommandError, config::Config};
use app_dirs::AppInfo;
use failure::Fail;
use structopt::StructOpt;

const APP_INFO: AppInfo = AppInfo {
    name: "dkr",
    author: "dkr",
};

const DKR_IMAGE: &str = "dkr";
const DKR_CONFIG_VOLUME: &str = "dkr-config";
const DOCKER_SOCKET_PATH: &str = "/var/run/docker.sock";

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {}", error);

        let dyn_error: &dyn Fail = &error;

        for cause in dyn_error.iter_causes() {
            eprintln!("       {}", cause);
        }
    }
}

#[derive(Debug, Fail)]
pub enum RunError {
    #[fail(display = "Failed to run command")]
    RunCommandError(#[cause] RunCommandError),

    #[fail(display = "Failed to run dkr inside a container")]
    RunContainerError(#[cause] io::Error),
}

fn run() -> Result<(), RunError> {
    let arguments = Arguments::from_args();
    let config = Config::load();

    if arguments.is_config_volume_enabled() && config_volume_exists() {
        run_in_container(arguments).map_err(RunError::RunContainerError)
    } else {
        arguments.run_command(config).map_err(RunError::RunCommandError)
    }
}

fn config_volume_exists() -> bool {
    docker::volume().inspect(DKR_CONFIG_VOLUME).is_ok()
}

fn run_in_container(arguments: Arguments) -> Result<(), io::Error> {
    let mut command = docker::run(DKR_IMAGE);

    command
        .temporary()
        .read_only_volume(DKR_CONFIG_VOLUME, "/root/.config/dkr")
        .volume(DOCKER_SOCKET_PATH, DOCKER_SOCKET_PATH);

    command.run_shell_command(format!(
        "dkr --disable-config-volume {}",
        arguments.to_string()
    ))
}
