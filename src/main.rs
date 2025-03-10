
mod services{
    pub mod json_loader;
    pub mod displayer;
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
use models::{aptitude::Aptitude, caracter::client::Client};
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

    let file_path = "assets/caracters/clients.json";

    let clients: Vec<Client> = match JsonLoader::loadJsonClients(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading clients: {}", e);
            return Ok(()); // ou un retour d'erreur spécifique
        }
    };

    // Affichage de la taille du vecteur clients
    println!("Number of clients: {}", clients.len());

    for client in clients.iter() {
        let pnj = &client.pnj;

        // Accéder correctement aux champs de 'caracter'
        println!("Client Name: {}", pnj.caracter.name);
        println!("Client Style: {}", pnj.caracter.style);
        println!("Client HP: {}", pnj.caracter.hp);
        println!("Client PP: {}", pnj.caracter.pp);
        println!("Client Job: {}", pnj.job);  // Le job est sur le 'Pnj'

        // Afficher les dialogues du client
        for dialog in &pnj.dialogs {
            println!("Dialog: {}", dialog);
        }

        // Afficher les conseils du client
        println!("Tips: {}", client.tips);
        for advice in &client.advices {
            println!("Advice: {}", advice);
        }

        println!("----------------------------");
    }


    
    
    Ok(())
}


