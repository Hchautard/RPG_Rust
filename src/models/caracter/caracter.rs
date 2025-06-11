use serde_derive::{Deserialize, Serialize};

/// Représente un personnage dans le jeu.
/// Un personnage a un nom, un style, des points de vie (hp), des points de pouvoir (pp) et un bankroll.
///
/// # Exemple
/// ```
/// let caracter = Caracter::new("Hero", "Warrior", 100, 50, 200);
/// assert_eq!(caracter.name, "Hero");
/// assert_eq!(caracter.style, "Warrior");
/// assert_eq!(caracter.hp, 100);
/// assert_eq!(caracter.pp, 50);
/// assert_eq!(caracter.bankroll, 200);
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Caracter {
    pub name: String,
    pub style: String,
    pub hp: i32,
    pub pp: i32,
    pub bankroll: i32,
}

impl Caracter {
    /// Crée un nouveau personnage avec un nom, un style, des points de vie, des points de pouvoir et un bankroll.
    ///
    /// # Arguments
    /// * `name` - Le nom du personnage.
    /// * `style` - Le style du personnage.
    /// * `hp` - Les points de vie du personnage.
    /// * `pp` - Les points de pouvoir du personnage.
    /// * `bankroll` - Le bankroll du personnage.
    ///
    /// # Returns
    /// Un nouveau `Caracter` avec les valeurs spécifiées.
    pub fn new(name: &str, style: &str, hp: i32, pp: i32, bankroll: i32) -> Self {
        Self {
            name: name.to_string(),
            style: style.to_string(),
            hp,
            pp,
            bankroll,
        }
    }
}
