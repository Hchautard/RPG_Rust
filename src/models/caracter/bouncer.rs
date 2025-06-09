use super::pnj::Pnj;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bouncer {
    pub pnj: Pnj,
    pub enigmas: Vec<String>,
}

impl Bouncer {
}