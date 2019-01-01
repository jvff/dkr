use super::super::{
    config::Config,
    docker_image::DockerImage,
    docker_image::{BuildDockerImageError, NewDockerImageError},
};
use failure::Fail;
use std::{collections::VecDeque, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Build {
    #[structopt(
        name = "images directory",
        short = "d",
        long = "base-dir",
        default_value = "/project",
        parse(from_os_str)
    )]
    images_dir: PathBuf,
    image_tag: String,
}

#[derive(Debug, Fail)]
pub enum RunBuildError {
    #[fail(
        display = "Missing image tag namespace in the config file or in the image tag: {}",
        _0
    )]
    NoTagNamespace(String),

    #[fail(display = "Failed to parse image description")]
    NewDockerImageError(#[cause] NewDockerImageError),

    #[fail(display = "Failed to build docker image")]
    BuildImageError(#[cause] BuildDockerImageError),
}

impl Build {
    pub fn run(self, config: Config) -> Result<(), RunBuildError> {
        let mut tag_queue = VecDeque::new();
        let mut build_queue = Vec::new();

        let tag_prefix = match config.tag_namespace {
            Some(namespace) => format!("{}/", namespace),
            None => {
                if let Some(end) = self.image_tag.find("/") {
                    self.image_tag[0..(end + 1)].to_owned()
                } else {
                    return Err(RunBuildError::NoTagNamespace(self.image_tag));
                }
            }
        };
        let tag_namespace = &tag_prefix[0..(tag_prefix.len() - 1)];

        let docker_image = DockerImage::new(&self.images_dir, &self.image_tag, tag_namespace)
            .map_err(RunBuildError::NewDockerImageError)?;

        tag_queue.extend(docker_image.source_images().map(|tag| tag.to_owned()));
        build_queue.push(docker_image);

        while let Some(source_image_tag) = tag_queue.pop_front() {
            if source_image_tag.starts_with(&tag_prefix) {
                let docker_image =
                    DockerImage::new(&self.images_dir, &source_image_tag, tag_namespace)
                        .map_err(RunBuildError::NewDockerImageError)?;

                tag_queue.extend(docker_image.source_images().map(|tag| tag.to_owned()));
                build_queue.push(docker_image);
            }
        }

        for docker_image in build_queue.into_iter().rev() {
            docker_image
                .build()
                .map_err(RunBuildError::BuildImageError)?;
        }

        Ok(())
    }
}
