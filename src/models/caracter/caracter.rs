use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Caracter {
    pub name: String,
    pub style: String,
    pub hp: i32,
    pub pp: i32,
    pub bankroll: i32,
}

impl Caracter {
    pub fn new(name: &str, style: &str, hp: i32, pp: i32, bankroll: i32) -> Self {
        Self {
            name: name.to_string(),
            style: style.to_string(),
            hp,
            pp,
            bankroll,
        }
    }
}
