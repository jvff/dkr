use failure::Fail;
use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer,
};
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

#[derive(Debug)]
pub struct RunCommands {
    commands: Vec<String>,
}

impl<'de> Deserialize<'de> for RunCommands {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(RunCommands {
            commands: deserializer.deserialize_any(SingleOrMultipleItemsVisitor)?,
        })
    }
}

impl Display for RunCommands {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let mut commands = self.commands.iter();

        if let Some(command) = commands.next() {
            write!(formatter, "RUN {}", command)?;

            for command in commands {
                write!(formatter, " && {}", command)?;
            }
        }

        writeln!(formatter)
    }
}

struct SingleOrMultipleItemsVisitor;

impl<'de> Visitor<'de> for SingleOrMultipleItemsVisitor {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "a string or a sequence of strings")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(vec![value.to_owned()])
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(vec![value.to_owned()])
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(vec![value])
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut elements = if let Some(size) = sequence.size_hint() {
            Vec::with_capacity(size)
        } else {
            Vec::new()
        };

        while let Some(element) = sequence.next_element()? {
            elements.push(element)
        }

        Ok(elements)
    }
}

#[derive(Debug, Deserialize)]
pub struct Dockerfile {
    from: String,
    workdir: Option<String>,
    user: Option<String>,
    add: Option<Vec<AddFile>>,
    env: Option<HashMap<String, String>>,
    run: RunCommands,
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

        self.run.fmt(formatter)
    }
}
