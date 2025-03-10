use std::fs::File;
use std::io::{Read, Write};
use serde_json;

use serde_json::json;
use crate::models::aptitude::Aptitude;
use crate::models::badge::Badge;
use crate::models::caracter::player::Player;
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

    pub fn load_json_aptitudes(file_path: &str) -> Result<Vec<Aptitude>, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let aptitudes: Vec<Aptitude> = serde_json::from_str(&data)?;
        Ok(aptitudes)
    }

    pub fn save_round(index: i32, player: &Player, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
       let json_data = json!({
            "index": index,
            "player": player
        });

        let mut file = File::create(file_path)?;

        file.write_all(json_data.to_string().as_bytes())?;

        Ok(())
    }

    pub fn load_round(file_path: &str) -> Result<(i32, Player), Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let json_data: serde_json::Value = serde_json::from_str(&data)?;

        let index = json_data["index"].as_i64().unwrap() as i32;
        let player: Player = serde_json::from_value(json_data["player"].clone())?;

        Ok((index, player))
    }

}
