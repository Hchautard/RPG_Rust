use crate::models::ingredient::Ingredient;

use super::pnj::Pnj;
use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Trader {
    pub pnj: Pnj,
    pub ingredients : Vec<Ingredient>
}

impl Trader {
    pub fn new(name: &str, style: &str, hp: i32, pp: i32, job: &str, dialogs: Vec<String>, ingredients: Vec<Ingredient>) -> Self {
        Self {
            pnj: Pnj::new(name, style, hp, pp, job, dialogs),
            ingredients
        }
    }

    pub fn buy() {
        // TODO
    }

    pub fn sell() {
        // TODO
    }
}
