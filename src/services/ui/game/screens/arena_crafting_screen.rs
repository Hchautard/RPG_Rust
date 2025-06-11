use bevy::prelude::*;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect};
use crate::services::ui::constants::NORMAL_BUTTON;
use crate::services::ui::game::{GameScreen, GameButtonAction, GameScreenState, ArenaUI};
use rand::seq::SliceRandom;
use rand::rng;

/// Affiche l'écran de phase de crafting de l'Arène.
/// Cet écran permet au joueur de réorganiser les instructions d'une recette pour vaincre le boss de l'Arène.
/// Il affiche les instructions dans un ordre aléatoire et permet de valider l'ordre choisi.
/// # Arguments
/// - `commands`: Les commandes pour créer des entités dans Bevy.
/// - `game_state`: L'état du jeu contenant les informations nécessaires pour l'écran de crafting.
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
        parent.spawn(Text::new("Dernière étape : remettre les instructions dans le bon ordre !"));

        if let Some(recipe) = &game_state.master_recipe {
            let mut shuffled_instructions = recipe.instructions.clone();
            shuffled_instructions.shuffle(&mut rng());

            parent.spawn(Text::new("Cliquez sur les étapes dans l'ordre :"));

            for (index, instruction) in shuffled_instructions.iter().enumerate() {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(400.0),
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
                    .with_child(Text::new(format!("Étape {} : {}", index + 1, instruction)));
            }

            parent.spawn(Text::new(format!(
                "Ordre sélectionné: {:?}",
                game_state.current_crafting.selected_instructions
            )));

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
                    GameButtonAction::ValidateInstructionOrder,
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child(Text::new("Valider l'ordre"));

            if game_state.current_crafting.instruction_correct {
                parent.spawn(Text::new("✅ Bravo, vous avez fini le boss !"));
            }
        } else {
            parent.spawn(Text::new("Aucune recette disponible."));
        }

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