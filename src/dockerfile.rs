use failure::Fail;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader},
    path::Path,
};

#[derive(Debug, Deserialize)]
pub struct AddFile {
    from: String,
    to: String,
}

#[derive(Debug, Deserialize)]
pub struct Dockerfile {
    from: String,
    add: Vec<AddFile>,
    workdir: Option<String>,
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
