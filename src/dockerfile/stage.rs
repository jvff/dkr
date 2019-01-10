use super::{
    add_file::AddFile, copy_file::CopyFile, packages::Packages, run_commands::RunCommands,
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stage {
    from: String,
    workdir: Option<String>,
    user: Option<String>,
    add: Option<Vec<AddFile>>,
    copy: Option<Vec<CopyFile>>,
    env: Option<HashMap<String, String>>,
    install: Option<Packages>,
    run: Option<RunCommands>,
    entrypoint: Option<String>,
    cmd: Option<String>,
}

impl Stage {
    pub fn from(&self) -> &str {
        &self.from
    }
}

impl Display for Stage {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        writeln!(formatter, "FROM {}", self.from)?;

        if let Some(workdir) = &self.workdir {
            writeln!(formatter, "WORKDIR {}", workdir)?;
        }

        if let Some(user) = &self.user {
            writeln!(formatter, "USER {}", user)?;
        }

        if let Some(add) = &self.add {
            add.iter()
                .map(|add_file| add_file.fmt(formatter))
                .find(Result::is_err)
                .unwrap_or(Ok(()))?;
        }

        if let Some(copy) = &self.copy {
            copy.iter()
                .map(|copy_file| copy_file.fmt(formatter))
                .find(Result::is_err)
                .unwrap_or(Ok(()))?;
        }

        if let Some(env) = &self.env {
            if env.len() > 0 {
                write!(formatter, "ENV")?;

                for (key, value) in env {
                    write!(formatter, " {}={}", key, value)?;
                }
            }

            writeln!(formatter)?;
        }

        if let Some(packages) = &self.install {
            packages.fmt(formatter)?;
        }

        if let Some(run_commands) = &self.run {
            run_commands.fmt(formatter)?;
        }

        if let Some(entrypoint) = &self.entrypoint {
            writeln!(formatter, "ENTRYPOINT {}", entrypoint)?;
        }

        if let Some(command) = &self.cmd {
            writeln!(formatter, "CMD {}", command)?;
        }

        Ok(())
    }
}
