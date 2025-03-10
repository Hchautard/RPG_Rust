use std::ptr::null;

use crate::models::{round::Round, aptitude::Aptitude, arena::Arena, badge::Badge, caracter::{bouncer::Bouncer, client::Client, master::Master, player::{self, Player}, trader::Trader}, ingredient::Ingredient};

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
    pub rounds: Vec<Round>
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
            rounds: Vec::new()
        }
    }

    pub fn init(&mut self){
        self.init_json();
        self.init_player();
        self.init_round();

    }

    pub fn init_json(&mut self){
    
        self.aptitudes = match JsonLoader::load_json_aptitudes("assets/caracters/aptitudes.json") {
            Ok(apt) => {
                apt
            },
            Err(e) => {
                vec![]
            }
        };

        self.traders = match JsonLoader::loadJsonTraders("assets/caracters/traders.json") {
            Ok(trader) => {
                trader
            },
            Err(e) => {
                vec![]
            }
        };

        self.badges = match JsonLoader::load_json_badges("assets/badges.json") {
            Ok(badges) => {
                badges
            },
            Err(e) => {
                vec![]
            }
        };
    }

    pub fn init_round(&mut self){
        for _ in 0..self.number_of_round {
            self.rounds.push(Round::new("round", "theme", "badge", "master", "bouncer", "trader", Vec::new()));
        }
    }
    pub fn init_player(&mut self){
        self.player= Some(Player::new("player", "style", Badge::new("badge", Vec::new()), Vec::new(), Vec::new()))
    }

    pub fn playRound(){


    }

    pub fn save(){
        
    }




}