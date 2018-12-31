mod docker_command;
mod docker_run;

pub use self::docker_run::DockerRun;
use std::borrow::Cow;

pub fn run<'a>(image: impl Into<Cow<'a, str>>) -> DockerRun<'a> {
    DockerRun::new(image)
}
