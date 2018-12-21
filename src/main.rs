mod dockerfile;

use self::dockerfile::Dockerfile;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Arguments {
    #[structopt(parse(from_os_str))]
    image: PathBuf,
}

fn main() {
    let arguments = Arguments::from_args();
    let mut dockerfiles = Vec::new();

    let dockerfile = Dockerfile::from_file(&arguments.image).unwrap();
    let mut template_image = dockerfile.from().to_owned();
    let base_dir = arguments
        .image
        .parent()
        .and_then(|image_dir| image_dir.parent())
        .unwrap_or_else(|| Path::new("."));

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
