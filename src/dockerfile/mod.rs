mod add_file;
mod packages;
mod run_commands;
mod single_or_multiple_items_visitor;

use self::{add_file::AddFile, packages::Packages, run_commands::RunCommands};
use failure::Fail;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    fs::File,
    io::{self, BufReader},
    path::Path,
};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dockerfile {
    from: String,
    workdir: Option<String>,
    user: Option<String>,
    add: Option<Vec<AddFile>>,
    env: Option<HashMap<String, String>>,
    install: Option<Packages>,
    run: RunCommands,
    cmd: Option<String>,
}

#[derive(Debug, Fail)]
pub enum FromFileError {
    #[fail(display = "IO error reading YAML dockerfile: {}", _0)]
    IoError(String, #[cause] io::Error),

    #[fail(display = "Failed to deserialize YAML dockerfile: {}", _0)]
    DeserializationError(String, #[cause] serde_yaml::Error),
}

impl Dockerfile {
    pub fn from_file(file_path: impl AsRef<Path>) -> Result<Self, FromFileError> {
        let file_path = file_path.as_ref();
        println!("Path: {}", file_path.display());
        let file = File::open(&file_path)
            .map_err(|error| FromFileError::IoError(file_path.display().to_string(), error))?;
        let reader = BufReader::new(file);

        serde_yaml::from_reader(reader).map_err(|error| {
            FromFileError::DeserializationError(file_path.display().to_string(), error)
        })
    }

    pub fn from(&self) -> &str {
        self.from.as_str()
    }
}

impl Display for Dockerfile {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        writeln!(formatter, "FROM {}", self.from)?;

        if let Some(workdir) = self.workdir.as_ref() {
            writeln!(formatter, "WORKDIR {}", workdir)?;
        }

        if let Some(user) = self.user.as_ref() {
            writeln!(formatter, "USER {}", user)?;
        }

        if let Some(add) = self.add.as_ref() {
            add.iter()
                .map(|add_file| add_file.fmt(formatter))
                .find(Result::is_err)
                .unwrap_or(Ok(()))?;
        }

        if let Some(env) = self.env.as_ref() {
            if env.len() > 0 {
                write!(formatter, "ENV")?;

                for (key, value) in env {
                    write!(formatter, " {}={}", key, value)?;
                }
            }

            writeln!(formatter)?;
        }

        if let Some(packages) = self.install.as_ref() {
            packages.fmt(formatter)?;
        }

        self.run.fmt(formatter)?;

        if let Some(command) = self.cmd.as_ref() {
            writeln!(formatter, "CMD {}", command)?;
        }

        Ok(())
    }
}
