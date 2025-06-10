use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

// Aptitude avec un nom, description, coût et puissance
pub struct Aptitude {
    pub name: String,
    pub description: String,
    pub pp: i32,
    pub power: f32,
}

impl Aptitude {}
