mod arguments;
mod config;
mod docker_image;
mod dockerfile;

use self::{arguments::Arguments, config::Config, docker_image::DockerImage};
use app_dirs::AppInfo;
use structopt::StructOpt;

const APP_INFO: AppInfo = AppInfo {
    name: "dkr",
    author: "dkr",
};

fn main() {
    let arguments = Arguments::from_args();
    let config = Config::load();

    let mut docker_images = Vec::new();

    let tag_prefix = config
        .tag_namespace
        .map(|prefix| format!("{}/", prefix))
        .unwrap_or_else(|| {
            if let Some(end) = arguments.image_tag.find("/") {
                arguments.image_tag[0..(end+1)].to_owned()
            } else {
                panic!("Missing image tage namespace")
            }
        });
    let tag_namespace = &tag_prefix[0..(tag_prefix.len() - 1)];

    let docker_image = DockerImage::new(&arguments.images_dir, &arguments.image_tag, tag_namespace).unwrap();
    let mut source_image_tag = docker_image.source_image().to_owned();

    docker_images.push(docker_image);

    while source_image_tag.starts_with(&tag_prefix) {
        let docker_image = DockerImage::new(&arguments.images_dir, &source_image_tag, tag_namespace).unwrap();

        source_image_tag = docker_image.source_image().to_owned();

        docker_images.push(docker_image);
    }

    for docker_image in docker_images.into_iter().rev() {
        docker_image.build().unwrap();
    }
}
