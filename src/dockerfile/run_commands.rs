use super::single_or_multiple_items_visitor::SingleOrMultipleItemsVisitor;
use serde::{Deserialize, Deserializer};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct RunCommands {
    commands: Vec<String>,
}

impl<'de> Deserialize<'de> for RunCommands {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(RunCommands {
            commands: deserializer.deserialize_any(SingleOrMultipleItemsVisitor)?,
        })
    }
}

impl Display for RunCommands {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let mut commands = self.commands.iter();

        if let Some(command) = commands.next() {
            write!(formatter, "RUN {}", command)?;

            for command in commands {
                write!(formatter, " && {}", command)?;
            }
        }

        writeln!(formatter)
    }
}
