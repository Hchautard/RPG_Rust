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
    pub fn new(
        name: &str,
        style: &str,
        hp: i32,
        pp: i32,
        job: &str,
        dialogs: Vec<String>,
        badge: Badge,
        attacks: Vec<String>,
        recipe: Recipe,
    ) -> Self {
        Self {
            pnj: Pnj::new(name, style, hp, pp, job, dialogs),
            badge,
            attacks,
            recipe,
        }
    }

    pub fn play() {
        // TODO
    }
}
