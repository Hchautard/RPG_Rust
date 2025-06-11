use bevy::prelude::*;
use crate::models::recipe::Recipe;

/// État du jeu, qui contient les informations sur l'écran actuel, les questions, les arènes, etc.
#[derive(Resource, Default)]
pub struct GameScreenState {
    pub current_screen: GameScreenType,
    pub current_question: String,
    pub answer_options: Vec<String>,
    pub correct_answer: String,
    pub available_arenas: Vec<(String, String)>,
    pub selected_arena: Option<String>,
    pub wrong_answer_message: bool,
    pub master_name: Option<String>,
    pub master_style: Option<String>,
    pub master_attacks: Vec<String>,
    pub master_dialogs: Vec<String>,
    pub master_badge: Option<String>,
    pub selected_arena_index: Option<usize>,
    pub arena_combat_state: ArenaCombatState,
    pub player_hp: i32,
    pub boss_hp: i32,
    pub show_intro_screen: bool,
    pub master_recipe: Option<Recipe>,
    pub current_boss_attack: Option<String>,
    pub current_crafting: CurrentCocktailCrafting,
    pub show_crafting_phase: bool,
}

/// Implémentation des méthodes pour l'état du jeu
impl GameScreenState {
    /// Réinitialise l'état du combat
    pub fn reset_combat(&mut self) {
        self.arena_combat_state = ArenaCombatState::Start;
        self.player_hp = 100;
        self.boss_hp = 100;
        self.show_intro_screen = true;
        self.show_crafting_phase = false;
        self.current_boss_attack = None;
        self.current_crafting = CurrentCocktailCrafting::default();
    }
}

/// Types d'écrans du jeu
#[derive(Default, PartialEq)]
pub enum GameScreenType {
    #[default]
    Main,
    ArenaSelection,
    ArenaPresentation,
    BouncerQuestion,
    Arena,
}

/// État du combat dans l'Arène
#[derive(Default, PartialEq)]
pub enum ArenaCombatState {
    #[default]
    Start,
    PlayerTurn,
    BossTurn,
    Victory,
    Defeat,
}

/// État de l'artisanat de cocktail en cours
#[derive(Default)]
pub struct CurrentCocktailCrafting {
    pub selected_ingredients: Vec<String>,
    pub completed: bool,
    pub correct: bool,
    pub selected_instructions: Vec<String>,
    pub instruction_correct: bool,
    pub selected_recipe: Option<Recipe>,
    pub cocktail_ready: bool,
}