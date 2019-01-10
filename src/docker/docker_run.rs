use super::docker_command::DockerCommand;
use std::{borrow::Cow, io};

pub struct DockerRun<'a> {
    image: Cow<'a, str>,
    command: DockerCommand<'a>,
}

impl<'a> DockerRun<'a> {
    pub fn new(image: impl Into<Cow<'a, str>>) -> Self {
        let mut command = DockerCommand::new();
        command.append("run");

        DockerRun {
            image: image.into(),
            command,
        }
    }

    pub fn temporary(&mut self) -> &mut Self {
        self.command.append("--rm");
        self
    }

    pub fn interactive(&mut self) -> &mut Self {
        self.command.append("-it");
        self
    }

    pub fn volume(&mut self, source: impl AsRef<str>, target: impl AsRef<str>) -> &mut Self {
        self.command
            .append("-v")
            .append(format!("{}:{}", source.as_ref(), target.as_ref()));
        self
    }

    pub fn read_only_volume(&mut self, source: impl AsRef<str>, target: impl AsRef<str>) -> &mut Self {
        self.command
            .append("-v")
            .append(format!("{}:{}:ro", source.as_ref(), target.as_ref()));
        self
    }

    pub fn env(&mut self, variable: impl AsRef<str>, value: impl AsRef<str>) -> &mut Self {
        self.command
            .append("-e")
            .append(format!("{}={}", variable.as_ref(), value.as_ref()));
        self
    }

    pub fn run(self) -> Result<(), io::Error> {
        let mut command = self.command;

        command.append(self.image);
        command.run()
    }

    pub fn run_shell_command(self, shell_command: impl AsRef<str>) -> Result<(), io::Error> {
        let mut command = self.command;

        command
            .append(self.image)
            .append("sh")
            .append("-c")
            .append(shell_command.as_ref());

        command.run()
    }
}
