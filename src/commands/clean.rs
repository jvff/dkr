use duct::cmd;
use failure::Fail;
use std::io;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Clean {}

#[derive(Debug, Fail)]
pub enum RunCleanError {
    #[fail(display = "Failed to list docker images")]
    ListImagesError(#[cause] io::Error),

    #[fail(display = "Failed to remove dangling docker images")]
    RemoveImagesError(#[cause] io::Error),
}

impl Clean {
    pub fn run(self) -> Result<(), RunCleanError> {
        let output = cmd!("docker", "images")
            .stdout_capture()
            .run()
            .map_err(RunCleanError::ListImagesError)?;
        let output_string = String::from_utf8_lossy(&output.stdout);
        let image_list = output_string.lines().skip(1);
        let dangling_images = image_list.filter_map(|image| {
            let mut image_info = image.split(" ").filter(|string| !string.is_empty());
            let repository = image_info.next();
            let tag = image_info.next();
            let image_id = image_info.next();

            if repository == Some("<none>") && tag == Some("<none>") {
                image_id
            } else {
                None
            }
        });

        let mut rmi_args = vec!["rmi"];

        rmi_args.extend(dangling_images);

        cmd("docker", &rmi_args)
            .run()
            .map_err(RunCleanError::RemoveImagesError)?;

        Ok(())
    }
}
