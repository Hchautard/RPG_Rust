use std::fs::File;
use std::io::Read;
use std::io::Write;
use serde_json;

use crate::models::aptitude::Aptitude;
use crate::models::badge::Badge;
use crate::models::caracter::client::Client;
use crate::models::caracter::trader::Trader;
use crate::models::arena::Arena;
use crate::models::caracter::bouncer::Bouncer;
use crate::models::caracter::master::Master;
use crate::models::caracter::player::Player;

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

    pub fn loadJsonMasters(file_path: &str)  -> Result<Vec<Master>, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let masters: Vec<Master> = serde_json::from_str(&data)?;
        Ok(masters)
    }

    pub fn loadJsonArena(file_path: &str) -> Result<Vec<Arena>, Box<dyn std::error::Error>>{
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let arena: Vec<Arena> = serde_json::from_str(&data)?;
        Ok(arena)
    }
    pub fn loadJsonBouncers(file_path: &str) -> Result<Vec<Bouncer>, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
    
        let bouncers: Vec<Bouncer> = serde_json::from_str(&data)?;
        Ok(bouncers)
    }

    pub fn loadJsonClients(file_path: &str) -> Result<Vec<Client>, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let clients: Vec<Client> = serde_json::from_str(&data)?;
        Ok(clients)
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

    pub fn save_player_to_json(file_path: &str, player: &Player) -> Result<(), Box<dyn std::error::Error>> {
        let mut players: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    
        // Lecture du fichier existant
        if let Ok(mut file) = File::open(file_path) {
            let mut data = String::new();
            file.read_to_string(&mut data)?;
            if !data.trim().is_empty() {
                // Désérialisation du fichier JSON en une Map
                players = serde_json::from_str(&data)?;
            }
        }
    
        let player_level = player.level.to_string();
    
        let player_json = serde_json::to_value(player)?;
        players.insert(player_level, player_json);
    
        let json = serde_json::to_string_pretty(&players)?;
        let mut file = File::create(file_path)?;
        file.write_all(json.as_bytes())?;
    
        Ok(())
    }

}
