use bevy::prelude::*;

/// Constantes de couleurs partagees
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const SELECTED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.75);
pub const RED: Color = Color::srgb(1.0, 0.0, 0.0);
pub const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
pub const BLUE: Color = Color::srgb(0.0, 0.0, 1.0);
pub const WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
pub const BLACK: Color = Color::srgb(0.0, 0.0, 0.0);

/// Etats principaux de l'application
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

/// Etats du jeu (sous-etats de AppState::Game)
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing
}

/// Actions des boutons pour les menus principaux
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

#[derive(Resource, Default)]
pub struct GameLoadContext {
    pub is_load_game: bool,
}