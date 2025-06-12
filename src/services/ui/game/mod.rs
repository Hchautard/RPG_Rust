pub mod components;
pub mod state;
pub mod screens;
pub mod systems;

// Exports publics pour maintenir la compatibilité
pub use components::*;
pub use state::*;
pub use systems::handle_game_button_actions;
pub use screens::main_screen::spawn_main_game_screen;

// Fonctions principales exposées
use bevy::prelude::*;
use crate::services::json_loader::JsonLoader;

pub fn setup_game(mut commands: Commands, mut game_state: ResMut<GameScreenState>) {
    // Initialiser l'état
    game_state.current_screen = GameScreenType::Main;
    
    // Charger les arènes depuis le JSON
    if let Ok(arenas) = JsonLoader::load_json_arena("assets/caracters/arena.json") {
        game_state.available_arenas = arenas.iter()
            .map(|arena| (arena.name.clone(), arena.theme.clone()))
            .collect();
    }
    
    // Charger les données du bouncer
    if let Ok(bouncers) = JsonLoader::load_json_bouncers("assets/caracters/pnj/bouncer.json") {
        if let Some(bouncer) = bouncers.first() {
            let question = bouncer.enigmas.first()
                .unwrap_or(&"Question par défaut".to_string())
                .clone();
            let options = vec![
                "L'homme".to_string(),
                "Un animal".to_string(),
                "Une machine".to_string(),
                "Le temps".to_string(),
            ];
            
            game_state.current_question = question;
            game_state.answer_options = options;
            game_state.correct_answer = "L'homme".to_string();
        }
    }

    spawn_main_game_screen(&mut commands);
}

pub fn despawn_game(mut commands: Commands, query: Query<Entity, With<GameScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}