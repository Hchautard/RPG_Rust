use crate::models::badge::Badge;

use super::pnj::Pnj;

use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Master {
    pub pnj: Pnj,
    pub badge : Badge,
    pub attacks: Vec<String>,
}

impl Master {
    pub fn new(name: &str, style: &str, hp: i32, pp: i32, job: &str, dialogs: Vec<String>, badge: Badge, attacks: Vec<String>) -> Self {
        Self {
            pnj: Pnj::new(name, style, hp, pp, job, dialogs),
            badge,
            attacks
        }
    }

    pub fn play(){
        // TODO
    }
}
