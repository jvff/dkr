use super::dockerfile::{self, Dockerfile};
use failure::Fail;
use std::path::{Path, PathBuf};

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

impl DockerImage {
    pub fn new(
        base_dir: impl AsRef<Path>,
        image_tag: impl AsRef<str>,
    ) -> Result<Self, NewDockerImageError> {
        let (image_tag, image_name) =
            Self::parse_image_tag(image_tag.as_ref()).map_err(NewDockerImageError::InvalidTag)?;
        let source_directory = base_dir.as_ref().join("janitovff").join(image_name);
        let dockerfile = Dockerfile::from_file(source_directory.join("dockerfile.yml"))
            .map_err(|error| NewDockerImageError::DockerfileError(image_tag.clone(), error))?;

        Ok(DockerImage {
            tag: image_tag,
            dockerfile,
            source_directory,
        })
    }

    fn parse_image_tag(image_tag: &str) -> Result<(String, String), String> {
        if image_tag.contains("/") {
            if image_tag.starts_with("janitovff/") {
                let (_, image_name) = image_tag.split_at(10);

                Ok((image_tag.to_owned(), image_name.to_owned()))
            } else {
                Err(image_tag.to_owned())
            }
        } else {
            let image_name = image_tag.to_owned();
            let mut image_tag = String::from("janitovff/");

            image_tag.push_str(&image_name);

            Ok((image_tag, image_name))
        }
    }

    pub fn source_image(&self) -> &str {
        self.dockerfile.from()
    }
}
