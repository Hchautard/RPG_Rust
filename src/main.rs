// Services
mod services {
    pub mod displayer_bevy;
    pub mod json_loader;
    pub mod ui;
}

// Modèle de données du jeu
mod models {
    pub mod aptitude;
    pub mod arena;
    pub mod badge;
    pub mod ingredient;
    pub mod recipe;
    pub mod caracter {
        pub mod bouncer;
        pub mod caracter;
        pub mod client;
        pub mod master;
        pub mod player;
        pub mod pnj;
        pub mod trader;
    }
}

use services::{displayer_bevy, json_loader::JsonLoader};
use std::io::{self};

// Point d'entrée principal du jeu
fn main() -> io::Result<()> {
    // Chemin pour le fichier des aptitudes
    let file_path = "assets/caracters/aptitudes.json";

    // On charge les aptitudes depuis le fichier JSON
    let aptitudes = match JsonLoader::load_json_aptitudes(file_path) {
        Ok(apt) => apt,
        Err(_e) => {
            // Si le fichier n'existe pas
            vec![]
        }
    };

    // On lance l'interface graphique
    let displayer_bevy = displayer_bevy::DisplayerBevy::new();
    let _ = displayer_bevy.run(&aptitudes);
    Ok(())
}
