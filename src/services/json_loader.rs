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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::tempdir;

    /// Test de chargement réussi d'une liste de maîtres depuis un fichier JSON
    /// Ce test vérifie que la fonction load_json_masters peut correctement charger
    /// et désérialiser des données de maîtres depuis un fichier JSON valide.
    /// Il teste également que les données sont correctement mappées aux champs de la structure Master.
    #[test]
    fn test_load_json_masters_success() {
        // Arrange
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_masters.json");
        let test_data = r#"[
            {
                "pnj": {
                    "caracter": {
                        "name": "Test Master",
                        "style": "Marseille",
                        "hp": 120,
                        "pp": 60,
                        "bankroll": 2000
                    },
                    "job": "Barman Test",
                    "dialogs": ["Hello test!"]
                },
                "badge": {
                    "name": "Test Badge",
                    "features": ["Test feature"]
                },
                "attacks": ["Test Attack"],
                "recipe": {
                    "ingredients": [],
                    "instructions": ["Test instruction"]
                }
            }
        ]"#;
        
        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_data.as_bytes()).unwrap();

        // Act
        let result = JsonLoader::load_json_masters(file_path.to_str().unwrap());

        // Assert
        assert!(result.is_ok());
        let masters = result.unwrap();
        assert_eq!(masters.len(), 1);
        assert_eq!(masters[0].pnj.caracter.name, "Test Master");
        assert_eq!(masters[0].pnj.job, "Barman Test");
    }

    /// Test de gestion d'erreur lors du chargement d'un fichier inexistant
    /// Ce test vérifie que la fonction load_json_masters retourne bien une erreur
    /// lorsqu'elle tente de charger un fichier qui n'existe pas, plutôt que de provoquer un panic.
    #[test]
    fn test_load_json_masters_file_not_found() {
        // Act
        let result = JsonLoader::load_json_masters("fichier_inexistant.json");

        // Assert
        assert!(result.is_err());
    }

    /// Test de chargement réussi d'une liste d'arènes depuis un fichier JSON
    /// Ce test vérifie que la fonction load_json_arena peut correctement charger
    /// et désérialiser des données d'arènes depuis un fichier JSON valide.
    /// Il valide le mapping des champs name et theme.
    #[test]
    fn test_load_json_arena_success() {
        // Arrange
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_arenas.json");
        let test_data = r#"[
            {
                "name": "TEST_ARENA",
                "theme": "Test Theme"
            }
        ]"#;
        
        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_data.as_bytes()).unwrap();

        // Act
        let result = JsonLoader::load_json_arena(file_path.to_str().unwrap());

        // Assert
        assert!(result.is_ok());
        let arenas = result.unwrap();
        assert_eq!(arenas.len(), 1);
        assert_eq!(arenas[0].name, "TEST_ARENA");
        assert_eq!(arenas[0].theme, "Test Theme");
    }

    /// Test de chargement réussi d'une liste de videurs depuis un fichier JSON
    /// Ce test vérifie que la fonction load_json_bouncers peut correctement charger
    /// et désérialiser des données de videurs avec leurs énigmes.
    /// Il teste le système d'énigmes stockées dans un tableau.
    #[test]
    fn test_load_json_bouncers_success() {
        // Arrange
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_bouncers.json");
        let test_data = r#"[
            {
                "pnj": {
                    "caracter": {
                        "name": "Test Bouncer",
                        "style": "Guardian",
                        "hp": 100,
                        "pp": 50,
                        "bankroll": 0
                    },
                    "job": "Doorman",
                    "dialogs": ["You shall not pass!"]
                },
                "enigmas": ["What has four legs but cannot walk?"]
            }
        ]"#;
        
        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_data.as_bytes()).unwrap();

        // Act
        let result = JsonLoader::load_json_bouncers(file_path.to_str().unwrap());

        // Assert
        assert!(result.is_ok());
        let bouncers = result.unwrap();
        assert_eq!(bouncers.len(), 1);
        assert_eq!(bouncers[0].pnj.caracter.name, "Test Bouncer");
        assert_eq!(bouncers[0].enigmas.len(), 1);
        assert_eq!(bouncers[0].enigmas[0], "What has four legs but cannot walk?");
    }

    /// Test de chargement réussi d'aptitudes depuis un fichier JSON
    /// Ce test vérifie que la fonction load_json_aptitudes peut correctement charger
    /// et désérialiser des données d'aptitudes depuis un fichier JSON valide.
    /// Il valide le mapping des champs name, description, pp et power.
    #[test]
    fn test_load_json_aptitudes_success() {
        // Arrange
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_aptitudes.json");
        let test_data = r#"[
            {
                "name": "Test Skill",
                "description": "A test skill for testing",
                "pp": 10,
                "power": 1.5
            }
        ]"#;
        
        let mut file = File::create(&file_path).unwrap();
        file.write_all(test_data.as_bytes()).unwrap();

        // Act
        let result = JsonLoader::load_json_aptitudes(file_path.to_str().unwrap());

        // Assert
        assert!(result.is_ok());
        let aptitudes = result.unwrap();
        assert_eq!(aptitudes.len(), 1);
        assert_eq!(aptitudes[0].name, "Test Skill");
        assert_eq!(aptitudes[0].description, "A test skill for testing");
        assert_eq!(aptitudes[0].pp, 10);
        assert_eq!(aptitudes[0].power, 1.5);
    }

    /// Test de sauvegarde réussie d'un joueur dans un nouveau fichier JSON
    /// Ce test vérifie que la fonction save_player_to_json peut correctement
    /// sérialiser et sauvegarder un joueur dans un fichier JSON en utilisant
    /// le niveau du joueur comme clé d'organisation des données.
    /// Il valide également que le fichier est créé et contient les bonnes informations.
    #[test]
    fn test_save_player_to_json_success() {
        // Arrange
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_player.json");
        
        // Créer un joueur de test avec la structure réelle basée sur le JSON
        let test_player = Player::new(
            "Test Player",
            "Hero",
            crate::models::badge::Badge { name: "Test Badge".to_string(), features: vec![] },
            vec![], // inventory vide
            vec![]  // aptitudes vides
        );

        // Act
        let result = JsonLoader::save_player_to_json(file_path.to_str().unwrap(), &test_player);

        // Assert
        assert!(result.is_ok());
        
        assert!(file_path.exists());
        let file_content = fs::read_to_string(&file_path).unwrap();
        assert!(file_content.contains("Test Player"));
        assert!(file_content.contains("\"1\""));  // level 1 par défaut
    }

    /// Test de mise à jour d'un fichier existant avec plusieurs joueurs
    /// Ce test vérifie que la fonction save_player_to_json peut correctement
    /// gérer l'ajout de plusieurs joueurs dans le même fichier JSON.
    /// Il teste la fonctionnalité de lecture d'un fichier existant, l'ajout de nouvelles données,
    /// et la réécriture du fichier avec toutes les informations préservées.
    #[test]
    fn test_save_player_update_existing_file() {
        // Arrange
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_player_update.json");
        
        let player1 = Player::new(
            "Player 1",
            "Warrior",
            crate::models::badge::Badge { name: "Badge 1".to_string(), features: vec![] },
            vec![],
            vec![]
        );
        
        let mut player2 = Player::new(
            "Player 2",
            "Mage", 
            crate::models::badge::Badge { name: "Badge 2".to_string(), features: vec![] },
            vec![],
            vec![]
        );
        player2.level = 2;

        // Act
        JsonLoader::save_player_to_json(file_path.to_str().unwrap(), &player1).unwrap();
        JsonLoader::save_player_to_json(file_path.to_str().unwrap(), &player2).unwrap();

        // Assert
        let file_content = fs::read_to_string(&file_path).unwrap();
        assert!(file_content.contains("Player 1"));
        assert!(file_content.contains("Player 2"));
        assert!(file_content.contains("\"1\""));
        assert!(file_content.contains("\"2\""));
    }

    /// Test de gestion d'erreur pour un format JSON invalide
    /// Ce test vérifie que les fonctions de chargement JSON gèrent correctement
    /// les erreurs de parsing lorsque le fichier contient du JSON mal formé.
    /// Il s'assure que l'erreur est propagée correctement sans provoquer de panic.
    #[test]
    fn test_load_json_invalid_format() {
        // Arrange
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("invalid.json");
        let invalid_data = "{ invalid json }";
        
        let mut file = File::create(&file_path).unwrap();
        file.write_all(invalid_data.as_bytes()).unwrap();

        // Act
        let result = JsonLoader::load_json_masters(file_path.to_str().unwrap());

        // Assert
        assert!(result.is_err());
    }

}
