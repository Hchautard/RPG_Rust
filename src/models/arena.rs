use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

/// Représente une arène dans le jeu
/// avec un nom et un thème.
///
/// # Exemple
/// ```
/// let arena = Arena {
///     name: "Marseille".to_string(),
///     theme: "Pastis".to_string(),
/// };
/// assert_eq!(arena.name, "Marseille");
/// assert_eq!(arena.theme, "Pastis");
/// ```
pub struct Arena {
    pub name: String,
    pub theme: String
}

impl Arena {
}