use crate::models::badge::Badge;

use super::{caracter::Caracter, pnj::Pnj};

use serde_derive::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Master {
    pub pnj: Pnj,
    pub badge : Badge,
    pub attacks: Vec<String>,
    pub max_hp: i32,
}

impl Master {
    pub fn new(name: &str, style: &str, hp: i32, max_hp:i32, pp: i32, job: &str, dialogs: Vec<String>, badge: Badge, attacks: Vec<String>) -> Self {
        Self {
            pnj: Pnj::new(name, style, hp, pp, job, dialogs),
            badge,
            attacks,
            max_hp
        }
    }

    pub fn play(){
        // TODO
    }
}

impl Default for Master {
    fn default() -> Self {
        Self {
            pnj: Pnj { caracter: Caracter::new("Boss", "default_style", 150, 30, 0), job: "Master".to_string(), dialogs: vec![] },
            badge: Badge::default(),
            attacks: vec![],
            max_hp: 150,
        }
    }
}
