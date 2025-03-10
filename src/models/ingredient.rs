use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub description : String,
    pub price: i32,
    pub type_ing : String // type Type
}

impl Ingredient {
    pub fn new(name: &str, description: String, price: i32, type_ing : String) -> Self {
        Self {
            name: name.to_string(),
            description,
            price,
            type_ing
        }
    }
}
