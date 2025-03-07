use std::fs::File;
use std::io::Read;
use serde_json;

use crate::models::badge::Badge;
use crate::models::caracter::trader::Trader;

pub struct JsonLoader {
}

impl JsonLoader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load_json_badges(file_path: &str) -> Result<Vec<Badge>, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let badges: Vec<Badge> = serde_json::from_str(&data)?;
        Ok(badges)
    }

    pub fn loadJsonMasters(){
    }

    pub fn loadJsonArena(){
    }
    pub fn loadJsonBouncers(){
    } 

    pub fn loadJsonClients(){

    }  
    pub fn loadJsonTraders(file_path: &str) -> Result<Vec<Trader>, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let traders: Vec<Trader> = serde_json::from_str(&data)?;
        Ok(traders)
    }
    pub fn loadJsonSpecializations(){
        
    }
}
