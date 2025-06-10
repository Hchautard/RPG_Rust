use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

// Un badge avec nom et liste de caractéristiques
pub struct Badge {
    pub name: String,
    pub features: Vec<String>,
}

impl Badge {}
