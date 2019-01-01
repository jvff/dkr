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

        if let Some(workdir) = self.workdir.as_ref() {
            writeln!(formatter, "WORKDIR {}", workdir)?;
        }

        if let Some(user) = self.user.as_ref() {
            writeln!(formatter, "USER {}", user)?;
        }

        if let Some(add) = self.add.as_ref() {
            add.iter()
                .map(|add_file| add_file.fmt(formatter))
                .find(Result::is_err)
                .unwrap_or(Ok(()))?;
        }

        if let Some(copy) = self.copy.as_ref() {
            copy.iter()
                .map(|copy_file| copy_file.fmt(formatter))
                .find(Result::is_err)
                .unwrap_or(Ok(()))?;
        }

        if let Some(env) = self.env.as_ref() {
            if env.len() > 0 {
                write!(formatter, "ENV")?;

                for (key, value) in env {
                    write!(formatter, " {}={}", key, value)?;
                }
            }

            writeln!(formatter)?;
        }

        if let Some(packages) = self.install.as_ref() {
            packages.fmt(formatter)?;
        }

        if let Some(run_commands) = self.run.as_ref() {
            run_commands.fmt(formatter)?;
        }

        if let Some(command) = self.cmd.as_ref() {
            writeln!(formatter, "CMD {}", command)?;
        }

        Ok(())
    }
}
