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
    pub images_dir: PathBuf,
    pub image_tag: String,
}
