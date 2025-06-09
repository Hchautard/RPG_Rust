
use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Aptitude {
    pub name: String,
    pub description: String,
    pub pp: i32,
    pub power: f32
}

impl Aptitude {
}