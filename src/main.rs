mod arguments;
mod commands;
mod config;
mod docker_environment;
mod docker_image;
mod dockerfile;

use self::{arguments::Arguments, config::Config};
use app_dirs::AppInfo;
use failure::Fail;
use structopt::StructOpt;

const APP_INFO: AppInfo = AppInfo {
    name: "dkr",
    author: "dkr",
};

fn main() {
    let arguments = Arguments::from_args();
    let config = Config::load();

    if let Err(error) = arguments.run_command(config) {
        eprintln!("Error: {}", error);

        let dyn_error: &dyn Fail = &error;

        for cause in dyn_error.iter_causes() {
            eprintln!("       {}", cause);
        }
    }
}
