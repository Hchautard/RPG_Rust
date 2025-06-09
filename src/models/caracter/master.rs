use crate::models::badge::Badge;
use crate::models::caracter::pnj::Pnj;
use crate::models::recipe::Recipe;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Master {
    pub pnj: Pnj,
    pub badge: Badge,
    pub attacks: Vec<String>,
    pub recipe: Recipe,
}

impl Master {
}
