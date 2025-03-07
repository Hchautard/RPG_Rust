use std::fs::File;
use std::io::Read;
use serde_json;

use crate::models::badge::Badge;

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
    pub fn loadJsonTraders(){

    }
    pub fn loadJsonSpecializations(){
        
    }
}
