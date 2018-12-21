use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Deserialize)]
pub struct AddFile {
    from: String,
    to: String,
}

impl Display for AddFile {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        writeln!(formatter, "ADD {} {}", self.from, self.to)
    }
}
