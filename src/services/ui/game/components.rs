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
    StartCombat,
    SelectIngredient(String),
    ValidateCocktail,
    StartArenaCombat,
    BackToMainFromCombat,
    StartFinalCraft,
    SelectInstruction(String),
    ValidateInstructionOrder,
}