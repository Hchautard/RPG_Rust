use bevy::ecs::system::Resource;

use crate::models::caracter::{master::Master, player::Player};

#[derive(Resource)]
pub struct CombatState {
    pub player: Player,
    pub master: Master,
    pub turn: Turn, 
    pub started: bool,
    pub finished: bool
}

#[derive(PartialEq)]
pub enum Turn {
    Player,
    Master,
}
