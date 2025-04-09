
mod services{
    pub mod json_loader;
    pub mod displayer;
    pub mod game;
    pub mod combat_state;
    pub mod displayer_bevy;
    // mod Json_loader; // Removed incorrect module declaration
}
mod models{
    pub mod badge;
    pub mod ingredient;
    pub mod combo;
    pub mod aptitude;
    pub mod round;
    pub mod fight;
    pub mod arena;
    pub mod caracter{
        pub mod caracter;
        pub mod bouncer;
        pub mod pnj;
        pub mod player;
        pub mod client;
        pub mod trader;
        pub mod master;
    }
}

use std::io::{self};
use services::{displayer_bevy, game::Game, json_loader::JsonLoader};
use models::{badge::Badge, caracter::player::Player};

fn main() -> io::Result<()> {
    let file_path = "assets/caracters/aptitudes.json";

    let aptitudes = match JsonLoader::load_json_aptitudes(file_path) {
        Ok(apt) => {
            apt
        },
        Err(e) => {
            vec![]
        }
    };

    
    // let mut displayer = Displayer::new()?;
    // displayer.show_menu(&aptitudes)?;
    // displayer.cleanup()?;
    let mut Game = services::game::Game::new(1);
    Game.init();
    Game.display();
    Ok(())
}


