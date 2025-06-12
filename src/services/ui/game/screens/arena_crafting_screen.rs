use bevy::prelude::*;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect};
use crate::services::ui::constants::NORMAL_BUTTON;
use crate::services::ui::game::{GameScreen, GameButtonAction, GameScreenState, ArenaUI};
use rand::seq::SliceRandom;
use rand::rng;

/// Affiche l'ecran de phase de crafting de l'Arène.
/// Cet ecran permet au joueur de reorganiser les instructions d'une recette pour vaincre le boss de l'Arène.
/// Il affiche les instructions dans un ordre aleatoire et permet de valider l'ordre choisi.
/// # Arguments
/// - `commands`: Les commandes pour creer des entites dans Bevy.
/// - `game_state`: L'etat du jeu contenant les informations necessaires pour l'ecran de crafting.
pub fn spawn_arena_crafting_phase_screen(commands: &mut Commands, game_state: &GameScreenState) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        GameScreen,
        ArenaUI,
    ))
    .with_children(|parent| {
        parent.spawn(Text::new("Derniere etape : remettre les instructions dans le bon ordre !"));

        // Afficher les HP
        parent.spawn(Text::new(format!("Votre HP: {}", game_state.player_hp)));
        parent.spawn(Text::new(format!("HP du Boss: {}", game_state.boss_hp)));

        if let Some(recipe) = &game_state.master_recipe {
            let mut shuffled_instructions = recipe.instructions.clone();
            shuffled_instructions.shuffle(&mut rng());

            parent.spawn(Text::new("Cliquez sur les etapes dans l'ordre :"));

            for (index, instruction) in shuffled_instructions.iter().enumerate() {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(500.0),
                            height: Val::Px(40.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        BorderColor(Color::BLACK),
                        BorderRadius::MAX,
                        BackgroundColor(NORMAL_BUTTON),
                        GameButtonAction::SelectInstruction(instruction.clone()),
                    ))
                    .with_child(Text::new(format!("Etape {} : {}", index + 1, instruction)));
            }

            // Afficher l'ordre selectionne avec plus de clarte
            if game_state.current_crafting.selected_instructions.is_empty() {
                parent.spawn(Text::new("Aucune instruction selectionnee"));
            } else {
                parent.spawn(Text::new("Ordre selectionne :"));
                for (i, instruction) in game_state.current_crafting.selected_instructions.iter().enumerate() {
                    parent.spawn(Text::new(format!("{}. {}", i + 1, instruction)));
                }
            }

            // Bouton de validation (actif seulement si on a selectionne des instructions)
            let validation_button_text = if game_state.current_crafting.selected_instructions.len() == recipe.instructions.len() {
                "Valider l'ordre".to_string()
            } else {
                format!("Sélectionnez {} instructions", recipe.instructions.len())
            };

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(50.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    GameButtonAction::ValidateInstructionOrder,
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child(Text::new(validation_button_text));

            // Bouton pour vider la selection
            if !game_state.current_crafting.selected_instructions.is_empty() {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(150.0),
                            height: Val::Px(40.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        GameButtonAction::ClearInstructions,
                        BorderColor(Color::BLACK),
                        BorderRadius::MAX,
                        BackgroundColor(Color::srgb(0.6, 0.3, 0.3)),
                    ))
                    .with_child(Text::new("Vider"));
            }

            // Feedback sur la dernière tentative
            if !game_state.current_crafting.instruction_correct && game_state.current_crafting.selected_instructions.is_empty() {
                parent.spawn(Text::new("Ordre incorrect ! Reessayez."));
            }
        } else {
            parent.spawn(Text::new("Aucune recette disponible."));
        }

        // Bouton retour
        parent
            .spawn((
                Button,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                GameButtonAction::BackToMainFromCombat,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child(Text::new("Retour"));
    });
}