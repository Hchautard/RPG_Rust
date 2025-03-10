
use serde_json::Value as JSON;

pub struct Combo {
    pub name: String,
    pub possibilities: JSON,
    pub theme: String,
}

impl Combo {
    pub fn new(name: &str, possibilities: JSON, theme: String) -> Self {
        Self {
            name: name.to_string(),
            possibilities,
            theme,
        }
    }

    pub fn verifyCombo(&self, ingredients: Vec<String>) -> bool {
        false
    }
    
}