
mod services{
    pub mod json_loader;
<<<<<<< HEAD
    pub mod displayer;
    mod Json_loader;
    pub mod game;
=======
>>>>>>> display
    pub mod displayer_bevy;
    // mod Json_loader; // Removed incorrect module declaration
    mod Json_loader;
    pub mod game;
<<<<<<< HEAD
=======
    mod ui;
>>>>>>> display
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
use services::{displayer_bevy, json_loader::JsonLoader};
use models::{badge::Badge, caracter::player::Player};
<<<<<<< HEAD
use services::{game::Game, json_loader::{self, JsonLoader}};
=======
// use services::{game::Game, json_loader::{self, JsonLoader}};
>>>>>>> display

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

<<<<<<< HEAD
    
=======
>>>>>>> display
    // let mut displayer = Displayer::new()?;
    // displayer.show_menu(&aptitudes)?;
    // displayer.cleanup()?;
    let mut displayer_bevy = displayer_bevy::DisplayerBevy::new();
    displayer_bevy.run(&aptitudes);
    Ok(())
}

/*

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
use models::{aptitude::Aptitude, caracter::client::Client,arena::Arena,caracter::bouncer::Bouncer, caracter::master::Master,caracter::player::Player, badge::Badge}; 
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

    let file_path = "assets/caracters/pnj/clients.json";

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

    let file_path = "assets/caracters/arena.json";

    let arenas: Vec<Arena> = match JsonLoader::loadJsonArena(file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading clients: {}", e);
            return Ok(());
        }
    };

    println!("Number of arenas: {}", arenas.len());

    for  a in arenas.iter() {
        println!("Arena Name: {}", a.name);
        println!("Arena Theme: {}", a.theme);
        println!("----------------------------");
    }

    let file_path = "assets/caracters/pnj/bouncer.json";

    let bouncers: Vec<Bouncer> = match JsonLoader::loadJsonBouncers(file_path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error loading bouncers: {}", e);
            return Ok(());
        }
    };

    println!("Number of bouncers: {}", bouncers.len());

    for b in bouncers.iter() {
        let pnj = &b.pnj;
    
        println!("Bouncer Name: {}", pnj.caracter.name);
        println!("Bouncer Style: {}", pnj.caracter.style);
        println!("Bouncer HP: {}", pnj.caracter.hp);
        println!("Bouncer PP: {}", pnj.caracter.pp);
        println!("Bouncer Job: {}", pnj.job);
    
        for dialog in &pnj.dialogs {
            println!("Dialog: {}", dialog);
        }
    
        for enigma in &b.enigmas {
            println!("Enigma: {}", enigma);
        }
    
        println!("----------------------------");
    }

    let file_path = "assets/caracters/pnj/masters.json";

    let masters: Vec<Master> = match JsonLoader::loadJsonMasters(file_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error loading masters: {}", e);
            return Ok(());
        }
    };

    println!("Number of masters: {}", masters.len());

    for m in masters.iter() {
        let pnj = &m.pnj;

        println!("Master Name: {}", pnj.caracter.name);
        println!("Master Style: {}", pnj.caracter.style);
        println!("Master HP: {}", pnj.caracter.hp);
        println!("Master PP: {}", pnj.caracter.pp);
        println!("Master Job: {}", pnj.job);

        for dialog in &pnj.dialogs {
            println!("Dialog: {}", dialog);
        }

        println!("Badge: {}", m.badge.name);
        for feature in &m.badge.features {
            println!("Feature: {}", feature);
        }

        for attack in &m.attacks {
            println!("Attack: {}", attack);
        }

        println!("----------------------------");
    }

    let mut player = Player::new("Ash Ketchum", "Pokémon Trainer", Badge {
        name: "Pikachu Badge".to_string(),
        features: vec!["Symbolizes victory over Pikachu".to_string()],
    }, vec![], vec![]);
    
    player.level = 2;
    
    match JsonLoader::save_player_to_json("assets/sauvegarde/players.json", &player) {
        Ok(()) => println!("Player saved successfully!"),
        Err(e) => eprintln!("Error saving player: {}", e),
    }
    
    
    


    Ok(())
}


 */

