use std::ptr::null;

use crate::models::{aptitude::Aptitude, arena::Arena, badge::Badge, caracter::{bouncer::Bouncer, client::Client, master::Master, player::Player, trader::Trader}, ingredient::Ingredient};

use super::Json_loader::JsonLoader;


pub struct Game {
    pub number_of_round: i32,
    pub bouncers: Vec<Bouncer>,
    pub clients: Vec<Client>,
    pub traders: Vec<Trader>,
    pub masters: Vec<Master>,
    pub aptitudes: Vec<Aptitude>,
    pub arena: Vec<Arena>, 
    pub player: Option<Player>,
    pub ingredients: Vec<Ingredient>,  
    pub badges: Vec<Badge>,
}


impl Game {
    pub fn new(number_of_round: i32) -> Self {
        Self {
            number_of_round,
            bouncers: Vec::new(),
            clients: Vec::new(),
            traders: Vec::new(),
            masters: Vec::new(),
            aptitudes: Vec::new(),
            arena: Vec::new(),
            player: None,
            ingredients: Vec::new(),
            badges:Vec::new(),
        }
    }

    pub fn init(&mut self){
        let file_path = "assets/caracters/aptitudes.json";
    
        self.aptitudes = match JsonLoader::load_json_aptitudes(file_path) {
            Ok(apt) => {
                apt
            },
            Err(e) => {
                vec![]
            }
        };
    }


}