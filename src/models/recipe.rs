use serde_derive::{Deserialize, Serialize};

use crate::models::ingredient::Ingredient;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub ingredients: Vec<Ingredient>,
    pub instructions: Vec<String>,
}

impl Recipe {}
