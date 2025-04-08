use bevy::prelude::*;
use crate::models::caracter::master::Master;
use crate::models::caracter::player::Player;

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
    Levels, 
    Fight,
    PlayerSlot,
    LevelsScreen,
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
    StartFight,
    SelectLevel(Level),  
    Locked,         
}


#[derive(Clone, PartialEq)]
pub struct Level {
    pub index: usize,
    pub name: String,
    pub image: Handle<Image>,
    pub is_locked: bool, 
    pub boss_name: String,
}

#[derive(Resource)]
pub struct LevelList {
    pub levels: Vec<Level>,
}

#[derive(Resource)]
pub struct MasterList {
    pub masters: Vec<Master>,
}

#[derive(Resource)]
pub struct PlayerResource {
    player: Player,
    CreatePlayer,
}