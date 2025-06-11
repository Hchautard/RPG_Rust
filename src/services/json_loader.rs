use serde_json;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use crate::models::aptitude::Aptitude;
use crate::models::arena::Arena;
use crate::models::caracter::bouncer::Bouncer;
use crate::models::caracter::master::Master;
use crate::models::caracter::player::Player;

/// Chargeur de données JSON pour le jeu
/// Ce module fournit des fonctions pour charger des données de jeu à partir de fichiers JSON,
pub struct JsonLoader {}

impl JsonLoader {
    /// Charge une liste de maîtres depuis le fichier JSON
    /// Cette fonction lit un fichier JSON contenant des maîtres et les charge dans une liste.
    /// Elle retourne un `Result` qui contient soit la liste de maîtres, soit une erreur.
    /// # Arguments
    /// * `file_path` - Le chemin du fichier JSON à charger.
    pub fn load_json_masters(file_path: &str) -> Result<Vec<Master>, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let masters: Vec<Master> = serde_json::from_str(&data)?;
        Ok(masters)
    }

    /// Charge une liste d'arènes depuis le fichier JSON
    /// Cette fonction lit un fichier JSON contenant des arènes et les charge dans une liste.
    /// Elle retourne un `Result` qui contient soit la liste d'arènes, soit une erreur.
    /// # Arguments
    /// * `file_path` - Le chemin du fichier JSON à charger.
    pub fn load_json_arena(file_path: &str) -> Result<Vec<Arena>, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let arena: Vec<Arena> = serde_json::from_str(&data)?;
        Ok(arena)
    }

    /// Charge une liste de videurs depuis le fichier JSON
    /// Cette fonction lit un fichier JSON contenant des videurs et les charge dans une liste.
    /// Elle retourne un `Result` qui contient soit la liste de videurs, soit une erreur.
    /// # Arguments
    /// * `file_path` - Le chemin du fichier JSON à charger.
    pub fn load_json_bouncers(file_path: &str) -> Result<Vec<Bouncer>, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let bouncers: Vec<Bouncer> = serde_json::from_str(&data)?;
        Ok(bouncers)
    }
    // Charge une liste d'aptitudes depuis le fichier JSON

    pub fn load_json_aptitudes(
        file_path: &str,
    ) -> Result<Vec<Aptitude>, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        let aptitudes: Vec<Aptitude> = serde_json::from_str(&data)?;
        Ok(aptitudes)
    }

    /// Sauvegarde le joueur dans un fichier JSON organisé par niveau
    /// Cette fonction lit un fichier JSON existant, ajoute ou met à jour le joueur avec son niveau comme clé,
    /// et enregistre le tout dans le fichier.
    /// # Arguments
    /// * `file_path` - Le chemin du fichier JSON où sauvegarder le joueur.
    /// * `player` - Le joueur à sauvegarder.
    pub fn save_player_to_json(
        file_path: &str,
        player: &Player,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut players: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();

        if let Ok(mut file) = File::open(file_path) {
            let mut data = String::new();
            file.read_to_string(&mut data)?;
            if !data.trim().is_empty() {
                players = serde_json::from_str(&data)?;
            }
        }
        // Ajoute le nouveau joueur avec son niveau comme clé

        let player_level = player.level.to_string();

        let player_json = serde_json::to_value(player)?;
        players.insert(player_level, player_json);

        let json = serde_json::to_string_pretty(&players)?;
        let mut file = File::create(file_path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    /// On crée le dossier de sauvegarde s'il n'existe pas
    /// Cette fonction vérifie si le dossier "save" existe, et le crée si nécessaire.
    /// Si la création échoue, elle affiche un message d'erreur.
    pub fn ensure_save_directory() {
        use std::fs;
        if let Err(e) = fs::create_dir_all("save") {
            println!("Erreur lors de la creation du dossier de sauvegarde: {}", e);
        }
    }
}
