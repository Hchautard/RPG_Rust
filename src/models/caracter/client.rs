use super::pnj::Pnj;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub pnj: Pnj,  
    pub tips: i32,
    pub advices: Vec<String>,
}

impl Client {
}
