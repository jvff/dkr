use super::dockerfile::{self, Dockerfile};
use duct::cmd;
use failure::Fail;
use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;

#[derive(Debug)]
pub struct DockerImage {
    tag: String,
    dockerfile: Dockerfile,
    source_directory: PathBuf,
}

#[derive(Debug, Fail)]
pub enum NewDockerImageError {
    #[fail(display = "Can't create image for invalid tag: {}", _0)]
    InvalidTag(String),

    #[fail(display = "Failed to load Dockerfile for image: {}", _0)]
    DockerfileError(String, #[cause] dockerfile::FromFileError),
}

#[derive(Debug, Fail)]
pub enum BuildDockerImageError {
    #[fail(
        display = "Failed to create file to write Dockerfile for image: {}",
        _0
    )]
    CreateDockerfileError(String, #[cause] io::Error),

    #[fail(display = "Failed to write Dockerfile contents for image: {}", _0)]
    WriteDockerfileError(String, #[cause] io::Error),

    #[fail(display = "Failed to run docker command to build image: {}", _0)]
    DockerCommandError(String, #[cause] io::Error),
}

impl DockerImage {
    pub fn new(
        base_dir: impl AsRef<Path>,
        image_tag: impl AsRef<str>,
        image_namespace: impl AsRef<str>,
    ) -> Result<Self, NewDockerImageError> {
        let (image_tag, image_name, image_namespace) =
            Self::parse_image_tag(image_tag.as_ref(), image_namespace.as_ref())
                .map_err(NewDockerImageError::InvalidTag)?;
        let source_directory = base_dir.as_ref().join(image_namespace).join(image_name);
        let dockerfile = Dockerfile::from_file(source_directory.join("dockerfile.yml"))
            .map_err(|error| NewDockerImageError::DockerfileError(image_tag.clone(), error))?;

        Ok(DockerImage {
            tag: image_tag,
            dockerfile,
            source_directory,
        })
    }

    fn parse_image_tag<'a, 'b>(
        image_tag: &'a str,
        image_namespace: &'b str,
    ) -> Result<(String, String, &'b str), String> {
        if let Some(position) = image_tag.find("/") {
            if image_tag.starts_with(image_namespace) && position == image_namespace.len() {
                let (_, image_name) = image_tag.split_at(position + 1);

                Ok((image_tag.to_owned(), image_name.to_owned(), image_namespace))
            } else {
                Err(image_tag.to_owned())
            }
        } else {
            let image_name = image_tag.to_owned();
            let mut image_tag = image_namespace.to_owned();

            image_tag.push('/');
            image_tag.push_str(&image_name);

            Ok((image_tag, image_name, image_namespace))
        }
    }

    pub fn source_images(&self) -> impl Iterator<Item = &str> {
        self.dockerfile.from()
    }

    pub fn build(&self) -> Result<(), BuildDockerImageError> {
        let dockerfile = NamedTempFile::new().map_err(|error| {
            BuildDockerImageError::CreateDockerfileError(self.tag.clone(), error)
        })?;

        write!(dockerfile.as_file(), "{}", self.dockerfile).map_err(|error| {
            BuildDockerImageError::WriteDockerfileError(self.tag.clone(), error)
        })?;

        cmd!(
            "docker",
            "build",
            "-t",
            &self.tag,
            "-f",
            dockerfile.path(),
            &self.source_directory
        )
        .run()
        .map(|_| ())
        .map_err(|error| BuildDockerImageError::DockerCommandError(self.tag.clone(), error))
    }
}
