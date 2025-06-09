use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Badge {
    pub name: String,
    pub features: Vec<String>,
}

impl Badge {
}