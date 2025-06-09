use super::caracter::Caracter;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pnj {
    pub caracter: Caracter,
    pub job: String,
    pub dialogs: Vec<String>,
}

impl Pnj {
}