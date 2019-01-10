mod docker_command;
mod docker_run;
mod docker_volume;

pub use self::{docker_run::DockerRun, docker_volume::DockerVolume};
use std::borrow::Cow;

pub fn run<'a>(image: impl Into<Cow<'a, str>>) -> DockerRun<'a> {
    DockerRun::new(image)
}

pub fn volume() -> DockerVolume {
    DockerVolume::new()
}
