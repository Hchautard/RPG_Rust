use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

// Une arène de combat avec nom et thème
pub struct Arena {
    pub name: String,
    pub theme: String
}

impl Arena {
}