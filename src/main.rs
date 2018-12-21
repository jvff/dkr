mod dockerfile;

use self::dockerfile::Dockerfile;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Arguments {
    #[structopt(parse(from_os_str))]
    image: PathBuf,
}

fn main() {
    let arguments = Arguments::from_args();
    let dockerfile = Dockerfile::from_file(&arguments.image).unwrap();

    println!("{}", dockerfile);
}
