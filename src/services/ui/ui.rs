// Module principal UI qui regroupe tous les sous-modules
pub mod constants;
pub mod main_menu;
pub mod aptitudes_screen;
pub mod game_screen;
pub mod player_slot_screen;
pub mod player_creation_screen;
pub mod start_screen;

// Re-export des composants et ressources fréquemment utilisés
pub use constants::{AppState, ButtonAction};
pub use player_slot_screen::{SelectedPlayerSlot, PlayerSlotScreenPlugin};