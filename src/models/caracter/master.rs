use crate::models::badge::Badge;
use crate::models::caracter::pnj::Pnj;
use crate::models::recipe::Recipe;

use serde_derive::{Deserialize, Serialize};

/// Représente un maître PNJ, qui est un personnage non-joueur avec des compétences de combat
/// et une recette à enseigner.
/// Il hérite des caractéristiques d'un PNJ et possède un badge, des attaques et une recette.
///
/// # Exemple
/// ```
/// let master = Master {
///     pnj: Pnj::new("Master", "A wise and powerful master"),
///     badge: Badge {
///         name: "Master Badge".to_string(),
///         features: vec!["Combat Expert".to_string(), "Recipe Master".to_string()],
///     },
///     attacks: vec!["Punch".to_string(), "Kick".to_string()],
///     recipe: Recipe {
///         name: "Master's Special Dish".to_string(),
///         ingredients: vec!["Ingredient1".to_string(), "Ingredient2".to_string()],
///         instructions: "Mix ingredients and cook for 30 minutes.".to_string(),
///     },
/// }; 
#[derive(Debug, Serialize, Deserialize)]
pub struct Master {
    pub pnj: Pnj,
    pub badge: Badge,
    pub attacks: Vec<String>,
    pub recipe: Recipe,
}

impl Master {
}
