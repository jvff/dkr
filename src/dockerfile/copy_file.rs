use serde::Deserialize;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Deserialize)]
pub struct CopyFile {
    from: String,
    to: String,
    stage: Option<usize>,
}

impl Display for CopyFile {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let stage = self
            .stage
            .map(|stage| format!("--from={} ", stage))
            .unwrap_or_else(|| String::new());

        writeln!(formatter, "COPY {}{} {}", stage, self.from, self.to)
    }
}
