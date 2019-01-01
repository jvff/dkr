mod add_file;
mod packages;
mod run_commands;
mod single_or_multiple_items_visitor;
mod stage;

use self::stage::Stage;
use failure::Fail;
use serde::Deserialize;
use serde_yaml::{Mapping, Number, Sequence, Value};
use std::{
    fmt::{self, Display, Formatter},
    fs, io,
    num::ParseFloatError,
    path::Path,
};
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dockerfile {
    stages: Vec<Stage>,
}

#[derive(Debug, Fail)]
pub enum FromFileError {
    #[fail(display = "IO error reading YAML dockerfile: {}", _0)]
    IoError(String, #[cause] io::Error),

    #[fail(display = "Failed to parse YAML dockerfile: {}", _0)]
    ParseYamlError(String, #[cause] ParseYamlError),

    #[fail(display = "Failed to deserialize YAML dockerfile: {}", _0)]
    DeserializationError(String, #[cause] serde_yaml::Error),
}

#[derive(Debug, Fail)]
pub enum ParseYamlError {
    #[fail(display = "Failed to parse YAML file")]
    YamlRustError(#[cause] yaml_rust::ScanError),

    #[fail(display = "Failed to parse YAML element")]
    ParseRealError(#[cause] ParseFloatError),

    #[fail(display = "Alias YAML elements are not supported")]
    AliasNotSupported,

    #[fail(display = "Attempt to access inexistent index or invalid type conversion")]
    BadYamlValue,
}

impl Dockerfile {
    pub fn from_file(file_path: impl AsRef<Path>) -> Result<Self, FromFileError> {
        let file_path = file_path.as_ref();
        let yaml_dockerfile = fs::read_to_string(&file_path)
            .map_err(|error| FromFileError::IoError(file_path.display().to_string(), error))?;
        let dockerfile_stages = YamlLoader::load_from_str(&yaml_dockerfile).map_err(|error| {
            FromFileError::ParseYamlError(
                file_path.display().to_string(),
                ParseYamlError::YamlRustError(error),
            )
        })?;

        let mut stages = Vec::with_capacity(dockerfile_stages.len());

        for dockerfile_stage in dockerfile_stages {
            let stage_value = convert_yaml_value(dockerfile_stage).map_err(|error| {
                FromFileError::ParseYamlError(file_path.display().to_string(), error)
            })?;
            let stage = serde_yaml::from_value(stage_value).map_err(|error| {
                FromFileError::DeserializationError(file_path.display().to_string(), error)
            })?;

            stages.push(stage);
        }

        Ok(Dockerfile { stages })
    }

    pub fn from(&self) -> impl Iterator<Item = &str> {
        self.stages.iter().map(Stage::from)
    }
}

impl Display for Dockerfile {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        for stage in &self.stages {
            stage.fmt(formatter)?;
            writeln!(formatter)?;
        }

        Ok(())
    }
}

fn convert_yaml_value(value: Yaml) -> Result<Value, ParseYamlError> {
    match value {
        Yaml::Real(string) => {
            let real: f64 = string.parse().map_err(ParseYamlError::ParseRealError)?;
            Ok(Value::Number(Number::from(real)))
        }
        Yaml::Integer(integer) => Ok(Value::Number(Number::from(integer))),
        Yaml::String(string) => Ok(Value::String(string)),
        Yaml::Boolean(boolean) => Ok(Value::Bool(boolean)),
        Yaml::Array(array) => {
            let elements: Result<Sequence, ParseYamlError> =
                array.into_iter().map(convert_yaml_value).collect();

            Ok(Value::Sequence(elements?))
        }
        Yaml::Hash(map) => {
            let elements: Result<Mapping, ParseYamlError> = map
                .into_iter()
                .map(|(key, value)| {
                    convert_yaml_value(key)
                        .and_then(|key| convert_yaml_value(value).map(|value| (key, value)))
                })
                .collect();

            Ok(Value::Mapping(elements?))
        }
        Yaml::Alias(_) => Err(ParseYamlError::AliasNotSupported),
        Yaml::Null => Ok(Value::Null),
        Yaml::BadValue => Err(ParseYamlError::BadYamlValue),
    }
}
