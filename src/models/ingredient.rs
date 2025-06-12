use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

// Ingrédient avec nom, description, prix et type
pub struct Ingredient {
    pub name: String,
    pub description: String,
    pub price: i32,
    pub type_ing: String,
}

impl Ingredient {}
