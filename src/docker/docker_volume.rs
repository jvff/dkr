use super::docker_command::DockerCommand;
use std::io;

pub struct DockerVolume;

impl DockerVolume {
    pub fn new() -> Self {
        DockerVolume
    }

    pub fn inspect(self, image: impl AsRef<str>) -> Result<(), io::Error> {
        let mut command = DockerCommand::new();

        command
            .append("volume")
            .append("inspect")
            .append(image.as_ref());

        command.run()
    }
}
