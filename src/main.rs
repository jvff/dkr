mod dockerfile;

use self::dockerfile::Dockerfile;
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
    let mut dockerfiles = Vec::new();
    let base_dir = arguments.images_dir.join("janitovff");

    let image_name = if arguments.image_tag.starts_with("janitovff/") {
        let (_, image_name) = arguments.image_tag.split_at(10);
        image_name
    } else {
        &arguments.image_tag
    };

    let image_dir = base_dir.join(image_name);
    let dockerfile = Dockerfile::from_file(image_dir.join("dockerfile.yml")).unwrap();
    let mut template_image = dockerfile.from().to_owned();

    dockerfiles.push(dockerfile);

    while template_image.starts_with("janitovff/") {
        let template_image_name = template_image.split_off(10);
        let dockerfile_path = base_dir.join(template_image_name).join("dockerfile.yml");
        println!("Deserializing {}", dockerfile_path.display());
        let dockerfile = Dockerfile::from_file(dockerfile_path).unwrap();

        template_image = dockerfile.from().to_owned();

        dockerfiles.push(dockerfile);
    }

    for dockerfile in dockerfiles.into_iter().rev() {
        println!("{}", dockerfile);
    }
}
