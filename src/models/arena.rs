use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Arena {
    pub name: String,
    pub theme: String
}

impl Arena {
}