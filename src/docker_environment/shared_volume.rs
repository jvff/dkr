use serde::Deserialize;

#[derive(Deserialize)]
pub struct SharedVolume {
    name: String,
    at: String,
}

impl SharedVolume {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn at(&self) -> &str {
        &self.at
    }
}
