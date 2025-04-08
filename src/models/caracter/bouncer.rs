use std::string;

use super::pnj::Pnj;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bouncer {
    pub pnj: Pnj,

    pub enigmas: Vec<String>,
}

impl Bouncer {
    pub fn new(
        name: &str,
        style: &str,
        hp: i32,
        pp: i32,
        job: &str,
        dialogs: Vec<String>,
        enigmas: Vec<String>,
    ) -> Self {
        Self {
            pnj: Pnj::new(name, style, hp, pp, job, dialogs),
            enigmas,
        }
    }

    pub fn give_enigma(&self, index: usize) -> Option<&String> {
        self.enigmas.get(index)
    }


    pub fn verify_enigma(&self, index: usize, answer: &str) -> bool {
        match self.enigmas.get(index) {
            Some(enigma) => enigma.to_lowercase().contains(&answer.to_lowercase()),
            None => false,
        }
    }

    pub fn bounce(&self) {
        println!("{} vous barre le passage !", self.pnj.caracter.name);
    }
}
