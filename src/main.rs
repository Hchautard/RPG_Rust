
mod services{
    pub mod json_loader;
    pub mod displayer;
    mod Json_loader;
    mod game;
}
mod models{
    pub mod badge;
    pub mod ingredient;
    pub mod combo;
    pub mod aptitude;
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
use crate::services::displayer::Displayer;
use models::aptitude::Aptitude;
use services::json_loader::JsonLoader;

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

    let mut displayer = Displayer::new()?;
    displayer.show_menu(&aptitudes)?;
    displayer.cleanup()?;
    Ok(())
}


