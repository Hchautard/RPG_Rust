// ================================
// constants.rs - Version mise à jour
// ================================

use bevy::prelude::*;

// Constantes de couleurs partagées
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
pub const SELECTED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.75);
pub const RED: Color = Color::srgb(1.0, 0.0, 0.0);
pub const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
pub const BLUE: Color = Color::srgb(0.0, 0.0, 1.0);
pub const WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
pub const BLACK: Color = Color::srgb(0.0, 0.0, 0.0);

// États principaux de l'application
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Aptitudes,
    Game,
    PlayerSlot,
    PlayerCreation,
    StartScreen,
}

// États du jeu (sous-états de AppState::Game)
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    BouncerQuestion,
    Arena,
}

// Actions des boutons pour les menus principaux
#[derive(Component, Clone)]
pub enum ButtonAction {
    NewGame,
    LoadGame,
    ShowAptitudes,
    Quit,
    Back,
    SelectSlot(usize),
    ConfirmSlot,
    CreatePlayer,
    StartGame,
}

// Actions spécifiques au jeu
#[derive(Component, Clone)]
pub enum GameAction {
    EncounterBouncer,
    AnswerQuestion(usize),
    BackToMenu,
    ContinueToArena,
}

#[derive(Resource, Default)]
pub struct GameLoadContext {
    pub is_load_game: bool,
}

// Ressource pour stocker les informations du bouncer et de sa question
#[derive(Resource)]
pub struct BouncerChallenge {
    pub bouncer_name: String,
    pub question: String,
    pub correct_answer: String,
    pub options: Vec<String>,
    pub is_answered_correctly: bool,
}

impl Default for BouncerChallenge {
    fn default() -> Self {
        Self {
            bouncer_name: "Bouncer Mystérieux".to_string(),
            question: "Qui marche sur quatre pattes le matin, deux pattes le midi, et trois pattes le soir ?".to_string(),
            correct_answer: "L'homme".to_string(),
            options: vec![
                "L'homme".to_string(),
                "Un animal".to_string(),
                "Une machine".to_string(),
                "Le temps".to_string(),
            ],
            is_answered_correctly: false,
        }
    }
}