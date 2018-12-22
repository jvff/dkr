mod docker_image;
mod dockerfile;

use self::docker_image::DockerImage;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Arguments {
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

fn main() {
    let arguments = Arguments::from_args();
    let mut docker_images = Vec::new();

    let docker_image = DockerImage::new(&arguments.images_dir, &arguments.image_tag).unwrap();

    let mut source_image_tag = docker_image.source_image().to_owned();

    docker_images.push(docker_image);

    while source_image_tag.starts_with("janitovff/") {
        let docker_image = DockerImage::new(&arguments.images_dir, &source_image_tag).unwrap();

        source_image_tag = docker_image.source_image().to_owned();

        docker_images.push(docker_image);
    }

    for docker_image in docker_images.into_iter().rev() {
        docker_image.build().unwrap();
    }
}
