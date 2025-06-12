use bevy::prelude::*;
use crate::services::ui::game::{GameScreenState, GameScreenType, screens::spawn_main_game_screen};
use crate::services::json_loader::JsonLoader;

/// Initialise l'écran de jeu.
/// Cette fonction est appelée pour configurer l'état initial du jeu,
/// charger les arènes et les données du bouncer, et afficher l'écran principal.
pub fn setup_game(mut commands: Commands, mut game_state: ResMut<GameScreenState>) {
    // Initialiser l'état
    game_state.current_screen = GameScreenType::Main;
    
    // Charger les arènes depuis le JSON
    if let Ok(arenas) = JsonLoader::loadJsonArena("assets/caracters/arena.json") {
        game_state.available_arenas = arenas.iter()
            .map(|arena| (arena.name.clone(), arena.theme.clone()))
            .collect();
    }
    
    // Charger les données du bouncer
    if let Ok(bouncers) = JsonLoader::loadJsonBouncers("assets/caracters/pnj/bouncer.json") {
        if let Some(bouncer) = bouncers.first() {
            let question = bouncer.enigmas.first()
                .unwrap_or(&"Question par defaut".to_string())
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

/// Supprime tous les éléments de l'écran de jeu.
/// Cette fonction est appelée pour nettoyer l'écran de jeu avant de le réinitialiser ou de le fermer.
pub fn despawn_game(mut commands: Commands, query: Query<Entity, With<GameScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}