use bevy::prelude::*;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect};
use crate::services::ui::constants::NORMAL_BUTTON;
use crate::services::ui::game::{GameScreen, GameButtonAction, GameScreenState, ArenaUI};

/// Affiche l'écran de fin d'Arène.
/// Cet écran est affiché lorsque le joueur a battu le boss de l'Arène.
/// Il affiche un message de félicitations, le nom du Maître de l'Arène et le nom de l'Arène.
/// # Arguments
/// - `commands`: Les commandes pour créer des entités dans Bevy.
/// - `game_state`: L'état du jeu contenant les informations nécessaires pour l'écran de fin d'Arène.
pub fn spawn_arena_end_screen(commands: &mut Commands, game_state: &GameScreenState) {
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
        parent.spawn(Text::new("🎉 Bravo ! Vous avez battu le boss ! 🏆"));

        parent.spawn(Text::new(format!(
            "Maître battu : {}\nArène : {}",
            game_state.master_name.as_deref().unwrap_or("???"),
            game_state.selected_arena.as_deref().unwrap_or("???"),
        )));

        parent
            .spawn((
                Button,
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(60.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                GameButtonAction::SelectArena,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child(Text::new("Retour à la sélection des niveaux"));
    });
}