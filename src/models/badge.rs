use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Badge {
    pub name: String,
    pub features: Vec<String>,
}

impl Badge {
    pub fn new(name: &str, features: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            features,
        }
    }
}



impl Default for Badge {
    fn default() -> Self {
        Self {
          name: "Default Badge".to_string(),
          features: vec!["Default Feature".to_string()],
        }
    }
}
