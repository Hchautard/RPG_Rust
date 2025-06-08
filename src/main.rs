
mod services{
    pub mod json_loader;
    pub mod displayer_bevy;
    mod Json_loader;
    pub mod game;
    mod ui;
}
mod models{
    pub mod badge;
    pub mod ingredient;
    pub mod combo;
    pub mod aptitude;
    pub mod round;
    pub mod fight;
    pub mod arena;
    pub mod recipe;
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
use services::{displayer_bevy, json_loader::JsonLoader};
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

    let mut displayer_bevy = displayer_bevy::DisplayerBevy::new();
    displayer_bevy.run(&aptitudes);
    Ok(())
}