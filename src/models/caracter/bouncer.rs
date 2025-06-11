use super::pnj::Pnj;
use serde_derive::{Deserialize, Serialize};

/// Représente un Bouncer, un PNJ qui garde l'entrée d'un lieu et pose des énigmes.
/// Il hérite des caractéristiques d'un PNJ et possède des énigmes à résoudre.
///
/// # Exemple
/// ```
/// let bouncer = Bouncer {
///     pnj: Pnj::new("Bouncer", "A tough-looking bouncer"),
///     enigmas: vec![
///         "What has keys but can't open locks?".to_string(),
///         "I speak without a mouth and hear without ears. What am I?".to_string(),
///     ],
/// };
/// assert_eq!(bouncer.pnj.name, "Bouncer");
/// assert_eq!(bouncer.enigmas.len(), 2);
/// ```
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bouncer {
    pub pnj: Pnj,
    pub enigmas: Vec<String>,
}

impl Bouncer {
}