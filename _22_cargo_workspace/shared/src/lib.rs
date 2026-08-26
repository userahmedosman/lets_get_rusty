use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectInfo {
    name: String,
    version: f32
}


impl ProjectInfo {
    pub fn show(name: String, version: f32) -> ProjectInfo {
        ProjectInfo{name, version}
    }
}