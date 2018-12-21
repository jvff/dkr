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
pub struct AddFile {
    from: String,
    to: String,
}

impl Display for AddFile {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        writeln!(formatter, "ADD {} {}", self.from, self.to)
    }
}

#[derive(Debug, Deserialize)]
pub struct Dockerfile {
    from: String,
    workdir: Option<String>,
    add: Vec<AddFile>,
    env: Option<HashMap<String, String>>,
    run: Vec<String>,
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
        let file = File::open(&file_path)
            .map_err(|error| FromFileError::IoError(file_path.display().to_string(), error))?;
        let reader = BufReader::new(file);

        serde_yaml::from_reader(reader).map_err(|error| {
            FromFileError::DeserializationError(file_path.display().to_string(), error)
        })
    }
}

impl Display for Dockerfile {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        writeln!(formatter, "FROM {}", self.from)?;

        if let Some(workdir) = self.workdir.as_ref() {
            writeln!(formatter, "WORKDIR {}", workdir)?;
        }

        self.add
            .iter()
            .map(|add_file| add_file.fmt(formatter))
            .find(Result::is_err)
            .unwrap_or(Ok(()))?;

        if let Some(env) = self.env.as_ref() {
            if env.len() > 0 {
                write!(formatter, "ENV")?;

                for (key, value) in env {
                    write!(formatter, " {}={}", key, value)?;
                }
            }

            writeln!(formatter)?;
        }

        let mut run = self.run.iter();

        if let Some(command) = run.next() {
            write!(formatter, "RUN {}", command)?;

            for command in run {
                write!(formatter, " && {}", command)?;
            }
        }

        Ok(())
    }
}
