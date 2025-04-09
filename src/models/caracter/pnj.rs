use super::caracter::Caracter;
use serde_derive::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pnj {
    pub caracter: Caracter,
    pub job: String,
    pub dialogs: Vec<String>,
}

impl Pnj {
    pub fn new(name: &str, style: &str, hp: i32, pp: i32, job: &str, dialogs: Vec<String>) -> Self {
        Self {
            caracter: Caracter::new(name, style, hp, pp, 0),
            job: job.to_string(),
            dialogs,
        }
    }

    pub fn talk(&self) {
        println!("{}: {}", self.caracter.name, self.dialogs[0]);
    }

}
