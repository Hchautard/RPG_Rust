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

// États de l'application
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Aptitudes,
    Game,
    PlayerSlot,
    PlayerCreation,
}

// Actions des boutons
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
}