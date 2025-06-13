use crate::services::json_loader::JsonLoader;
use crate::services::ui::constants::NORMAL_BUTTON;
use crate::services::ui::game::{
    screens::*, ArenaUI, GameButtonAction, GameScreen, GameScreenState, GameScreenType,
};
use bevy::prelude::*;
use std::collections::HashSet;

/// Gère les actions des boutons dans l'interface du jeu.
///
/// Cette fonction est appelée à chaque frame pour vérifier les interactions
/// avec les boutons du jeu. Elle met à jour l'état du jeu en fonction des actions
/// des boutons et change la couleur de fond des boutons en fonction de leur état.
pub fn handle_game_button_actions(
    mut interaction_query: Query<
        (&Interaction, &GameButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
    mut game_state: ResMut<GameScreenState>,
    game_entities: Query<Entity, With<GameScreen>>,
    arena_ui_query: Query<Entity, With<ArenaUI>>,
) {
    for (interaction, action, mut background_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                handle_button_press(
                    action,
                    &mut commands,
                    &mut game_state,
                    &game_entities,
                    &arena_ui_query,
                );
                *background_color = Color::srgb(0.3, 0.3, 0.5).into();
            }
            Interaction::Hovered => {
                *background_color = Color::srgb(0.25, 0.25, 0.35).into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}

// Fonction pour gérer les actions des boutons
// Cette fonction est appelée lorsque l'utilisateur interagit avec un bouton.
// Elle met à jour l'état du jeu en fonction de l'action du bouton et gère les transitions d'écran.
fn handle_button_press(
    action: &GameButtonAction,
    commands: &mut Commands,
    game_state: &mut ResMut<GameScreenState>,
    game_entities: &Query<Entity, With<GameScreen>>,
    arena_ui_query: &Query<Entity, With<ArenaUI>>,
) {
    // Nettoie l'écran actuel
    let mut clear_screen = || {
        for entity in game_entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
    };

    match action {
        GameButtonAction::SelectArena => {
            game_state.wrong_answer_message = false;
            clear_screen();
            game_state.current_screen = GameScreenType::ArenaSelection;
            spawn_arena_selection_screen(commands, game_state);
        }
        GameButtonAction::ChooseArena(arena_index) => {
            handle_arena_choice(commands, game_state, game_entities, *arena_index);
        }
        GameButtonAction::AnswerQuestion(answer_index) => {
            handle_bouncer_answer(commands, game_state, game_entities, *answer_index);
        }
        GameButtonAction::EncounterBouncer => {
            clear_screen();
            game_state.current_screen = GameScreenType::Arena;
            game_state.arena_combat_state =
                crate::services::ui::game::state::ArenaCombatState::Start;
            game_state.player_hp = 100;
            game_state.boss_hp = 100;
            game_state.current_crafting =
                crate::services::ui::game::state::CurrentCocktailCrafting::default();
            game_state.show_intro_screen = true;
            spawn_arena_combat_screen(commands, game_state);
        }
        GameButtonAction::StartArenaCombat => {
            game_state.show_intro_screen = false;
            for entity in arena_ui_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            spawn_arena_combat_screen(commands, game_state);
        }
        GameButtonAction::SelectIngredient(ingredient) => {
            if !game_state
                .current_crafting
                .selected_ingredients
                .contains(ingredient)
            {
                game_state
                    .current_crafting
                    .selected_ingredients
                    .push(ingredient.clone());
            } else {
                game_state
                    .current_crafting
                    .selected_ingredients
                    .retain(|i| i != ingredient);
            }
            clear_screen();
            spawn_arena_combat_screen(commands, game_state);
        }
        GameButtonAction::ValidateCocktail => {
            // Valide le cocktail en comparant avec la recette du maître
            let is_cocktail_correct = validate_cocktail_recipe(game_state);

            if is_cocktail_correct {
                // Cocktail correct
                game_state.current_crafting.cocktail_ready = true;
                game_state.show_crafting_phase = true;
                game_state.show_intro_screen = false;

                // Vider la sélection d'ingrédients pour la phase suivante
                game_state.current_crafting.selected_ingredients.clear();

                // Rafraîchir l'écran pour afficher la phase de crafting
                for entity in arena_ui_query.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                spawn_arena_combat_screen(commands, game_state);
            } else {
                // Cocktail incorrect
                game_state.player_hp = game_state.player_hp.saturating_sub(20); // Réduire HP de 20
                game_state.current_crafting.selected_ingredients.clear(); // Vider la sélection

                // Vérifie si le joueur a perdu
                if game_state.player_hp == 0 {
                    // Joueur défait
                    game_state.current_crafting.cocktail_ready = false;
                    clear_screen();
                    spawn_arena_end_screen(commands, game_state);
                } else {
                    // Continuer le combat
                    clear_screen();
                    spawn_arena_combat_screen(commands, game_state);
                }
            }
        }
        GameButtonAction::BackToMainFromCombat => {
            clear_screen();
            game_state.current_screen = GameScreenType::Main;
            spawn_main_game_screen(commands);
        }
        GameButtonAction::StartFinalCraft => {
            game_state.show_crafting_phase = false;
            for entity in arena_ui_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            spawn_arena_crafting_phase_screen(commands, game_state);
        }
        GameButtonAction::SelectInstruction(instruction) => {
            if !game_state
                .current_crafting
                .selected_instructions
                .contains(instruction)
            {
                game_state
                    .current_crafting
                    .selected_instructions
                    .push(instruction.to_string());
            }

            for entity in arena_ui_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            spawn_arena_crafting_phase_screen(commands, game_state);
        }
        GameButtonAction::ValidateInstructionOrder => {
            if let Some(recipe) = &game_state.master_recipe {
                let expected = &recipe.instructions;
                let selected = &game_state.current_crafting.selected_instructions;

                if selected == expected {
                    // Ordre correct
                    game_state.current_crafting.instruction_correct = true;
                    game_state.boss_hp = 0;
                    for entity in arena_ui_query.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                    spawn_arena_end_screen(commands, game_state);
                } else {
                    // Ordre incorrect
                    let damage = 15;

                    // Calculer les nouveaux HP avec vérification de défaite
                    if game_state.player_hp <= damage {
                        // Le joueur va mourir
                        game_state.player_hp = 0;

                        for entity in arena_ui_query.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                        spawn_arena_end_screen(commands, game_state);
                    } else {
                        // Le joueur survit
                        game_state.player_hp -= damage;
                        game_state.current_crafting.selected_instructions.clear();
                        game_state.current_crafting.instruction_correct = false;

                        for entity in arena_ui_query.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                        spawn_arena_crafting_phase_screen(commands, game_state);
                    }
                }
            }
        }
        GameButtonAction::BackToMainGame => {
            clear_screen();
            game_state.current_screen = GameScreenType::Main;
            spawn_main_game_screen(commands);
        }
        GameButtonAction::BackToArenaSelection => {
            game_state.wrong_answer_message = false;
            clear_screen();
            game_state.current_screen = GameScreenType::ArenaSelection;
            spawn_arena_selection_screen(commands, game_state);
        }
        GameButtonAction::StartCombat => {
            clear_screen();
            game_state.current_screen = GameScreenType::Arena;
            game_state.reset_combat();
            spawn_arena_combat_screen(commands, game_state);
        }
        GameButtonAction::ClearInstructions => {
            game_state.current_crafting.selected_instructions.clear();
            for entity in arena_ui_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            spawn_arena_crafting_phase_screen(commands, game_state);
        }
    }
}

/// Gère le choix de l'arène sélectionnée par l'utilisateur.
/// Cette fonction met à jour l'état du jeu avec l'arène sélectionnée et
/// prépare l'écran pour la question du bouncer.
fn handle_arena_choice(
    commands: &mut Commands,
    game_state: &mut ResMut<GameScreenState>,
    game_entities: &Query<Entity, With<GameScreen>>,
    arena_index: usize,
) {
    game_state.wrong_answer_message = false;

    if let Some((arena_name, _)) = game_state.available_arenas.get(arena_index) {
        game_state.selected_arena = Some(arena_name.clone());
        game_state.selected_arena_index = Some(arena_index);
    }

    for entity in game_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    game_state.current_screen = GameScreenType::BouncerQuestion;
    spawn_bouncer_question_screen(commands, game_state);
}

/// Gère la réponse du joueur à la question du bouncer.
/// Cette fonction vérifie si la réponse est correcte et met à jour l'état du jeu en conséquence.
fn handle_bouncer_answer(
    commands: &mut Commands,
    game_state: &mut ResMut<GameScreenState>,
    game_entities: &Query<Entity, With<GameScreen>>,
    answer_index: usize,
) {
    let selected_answer = &game_state.answer_options[answer_index];

    for entity in game_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    if *selected_answer == game_state.correct_answer {
        load_master_data(game_state);
        game_state.current_screen = GameScreenType::ArenaPresentation;
        spawn_arena_presentation_screen(commands, game_state);
    } else {
        game_state.wrong_answer_message = true;
        game_state.current_screen = GameScreenType::ArenaSelection;
        spawn_arena_selection_screen(commands, game_state);
    }
}

/// Charge les données du maître (master) sélectionné dans l'état du jeu.
fn load_master_data(game_state: &mut GameScreenState) {
    match JsonLoader::load_json_masters("assets/caracters/pnj/masters.json") {
        Ok(masters) => {
            if let Some(selected_index) = game_state.selected_arena_index {
                if let Some(master) = masters.get(selected_index) {
                    game_state.master_name = Some(master.pnj.caracter.name.clone());
                    game_state.master_style = Some(master.pnj.caracter.style.clone());
                    game_state.master_badge = Some(master.badge.name.clone());
                    game_state.master_attacks = master.attacks.clone();
                    game_state.master_dialogs = master.pnj.dialogs.clone();
                    game_state.master_recipe = Some(master.recipe.clone());
                }
            }
        }
        Err(e) => {
            info!("Erreur lors du chargement des masters : {:?}.", e);
        }
    }
}

/// Valide si les ingrédients sélectionnés correspondent à la recette du maître
fn validate_cocktail_recipe(game_state: &GameScreenState) -> bool {
    if let Some(recipe) = &game_state.master_recipe {
        let selected_ingredients: std::collections::HashSet<String> = game_state
            .current_crafting
            .selected_ingredients
            .iter()
            .cloned()
            .collect();

        let expected_ingredients: std::collections::HashSet<String> =
            recipe.ingredients.iter().map(|i| i.name.clone()).collect();

        // Vérifie que les ingrédients sélectionnés correspondent exactement à la recette
        selected_ingredients == expected_ingredients
    } else {
        false // Pas de recette = échec
    }
}
