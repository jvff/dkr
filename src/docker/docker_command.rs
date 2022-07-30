use duct::cmd;
use std::{
    borrow::{Borrow, Cow},
    ffi::OsStr,
    io,
};

pub struct DockerCommand<'a> {
    arguments: Vec<Cow<'a, str>>,
}

impl<'a> DockerCommand<'a> {
    pub fn new() -> Self {
        DockerCommand {
            arguments: Vec::new(),
        }
    }

    pub fn append(&mut self, argument: impl Into<Cow<'a, str>>) -> &mut Self {
        self.arguments.push(argument.into());
        self
    }

    pub fn run(self) -> Result<(), io::Error> {
        cmd(
            "docker",
            self.arguments.iter().map(|argument| {
                let argument_str: &str = argument.borrow();
                OsStr::new(argument_str)
            }),
        )
        .run()
        .map(|_| ())
    }
}
