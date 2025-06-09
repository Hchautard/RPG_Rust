use crate::models::ingredient::Ingredient;

use super::pnj::Pnj;
use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Trader {
    pub pnj: Pnj,
    pub ingredients : Vec<Ingredient>
}

impl Trader {
}
