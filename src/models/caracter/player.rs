use crate::models::{aptitude::Aptitude, badge::Badge, ingredient::Ingredient};
use super::caracter::Caracter;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub caracter: Caracter,
    pub badge: Badge,
    pub inventory: Vec<Ingredient>,
    pub level: u32,
    pub reputation: u32,
    pub aptitudes: Vec<Aptitude>,
}

impl Player {
    pub fn new(name: &str, style: &str, badge: Badge, inventory: Vec<Ingredient>, aptitudes: Vec<Aptitude>) -> Self {
        Self {
            caracter: Caracter::new(name, style, 100, 50, 0),
            level: 1,
            reputation: 0,
            inventory,
            badge,
            aptitudes,
        }
    }

    pub fn gain_xp(&mut self, xp: u32) {
        self.reputation += xp;
        println!("{} gagne {} XP!", self.caracter.name, xp);

        if self.reputation >= 100 {
            self.level_up();
        }
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.reputation = 0;
        println!("{} passe au niveau {}!", self.caracter.name, self.level);
    }
}