
mod services{
    pub mod json_loader;
    pub mod displayer_bevy;
    pub mod ui;
}
mod models{
    pub mod badge;
    pub mod ingredient;
    pub mod aptitude;
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

fn main() -> io::Result<()> {
    let file_path = "assets/caracters/aptitudes.json";

    let aptitudes = match JsonLoader::load_json_aptitudes(file_path) {
        Ok(apt) => {
            apt
        },
        Err(_e) => {
            vec![]
        }
    };

    let displayer_bevy = displayer_bevy::DisplayerBevy::new();
    let _ = displayer_bevy.run(&aptitudes);
    Ok(())
}