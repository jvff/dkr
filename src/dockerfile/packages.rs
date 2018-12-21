use super::single_or_multiple_items_visitor::SingleOrMultipleItemsVisitor;
use serde::{Deserialize, Deserializer};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Packages {
    packages: Vec<String>,
}

impl<'de> Deserialize<'de> for Packages {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Packages {
            packages: deserializer.deserialize_any(SingleOrMultipleItemsVisitor)?,
        })
    }
}

impl Display for Packages {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(
            formatter,
            r#"RUN if [ "$UID" -eq 0 ]; then apt-get update -y; else sudo apt-get update -y; fi"#
        )?;

        if self.packages.len() > 0 {
            write!(
                formatter,
                r#" && if [ "$UID" -eq 0 ]; then \
                    apt-get install -y {packages}; \
                else \
                    sudo apt-get install -y {packages}; \
                fi"#,
                packages = self.packages.join(" "),
            )?;
        }

        writeln!(formatter)
    }
}
