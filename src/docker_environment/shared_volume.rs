use serde::Deserialize;

#[derive(Deserialize)]
pub struct SharedVolume {
    name: String,
    at: String,
}

impl SharedVolume {
    pub fn volume_argument(&self) -> String {
        format!("{}:{}", self.name, self.at)
    }
}
