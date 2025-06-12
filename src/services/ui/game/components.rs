use bevy::prelude::*;

#[derive(Component)]
pub struct GameScreen;

#[derive(Component)]
pub struct BouncerQuestionUI;

#[derive(Component)]
pub struct ArenaUI;

#[derive(Component)]
pub struct ArenaPresentationUI;

#[derive(Component, Clone)]
pub enum GameButtonAction {
    SelectArena,
    ChooseArena(usize),
    EncounterBouncer,
    AnswerQuestion(usize),
    BackToMainGame,
    BackToArenaSelection,
    SelectIngredient(String),
    ValidateCocktail,
    StartArenaCombat,
    BackToMainFromCombat,
    StartCombat,
    StartFinalCraft,
    SelectInstruction(String),
    ValidateInstructionOrder,
    ClearInstructions
}