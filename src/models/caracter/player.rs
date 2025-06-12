use crate::models::{aptitude::Aptitude, badge::Badge, ingredient::Ingredient};
use super::caracter::Caracter;
use serde_derive::{Deserialize, Serialize};

/// Représente un joueur dans le jeu
/// 
/// # Exemple
/// 
/// ```
/// let player = Player::new("Alice", 100);
/// assert_eq!(player.name(), "Alice");
/// ```
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
    /// Crée un nouveau joueur avec un nom et des points de vie
    /// 
    /// # Arguments
    /// 
    /// * `name` - Le nom du joueur
    /// * `style` - Le style du joueur
    /// * `badge` - Le badge du joueur
    /// * `inventory` - L'inventaire du joueur
    /// * `aptitudes` - Les aptitudes du joueur
    /// 
    /// # Returns
    /// 
    /// Un nouveau `Player` avec les valeurs spécifiées
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
}