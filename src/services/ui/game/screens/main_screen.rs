use bevy::prelude::*;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect};
use crate::services::ui::constants::{ButtonAction, NORMAL_BUTTON};
use crate::services::ui::game::{GameScreen, GameButtonAction};

/// Affiche l'écran principal du jeu.
/// Cet écran permet de sélectionner une arène pour commencer un combat.
/// Il contient également un bouton pour revenir à l'écran précédent.
///
/// # Arguments
/// - `commands`: Les commandes pour créer des entités dans Bevy.
pub fn spawn_main_game_screen(commands: &mut Commands) {
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
        BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        GameScreen,
    ))
    .with_children(|parent| {
        parent.spawn(Text::new(""));
        
        // Bouton pour sélectionner une arène
        parent
            .spawn((
                Button,
                Node {
                    width: Val::Px(250.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
                GameButtonAction::SelectArena,
            ))
            .with_child(Text::new("Selectionner une Arene"));

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
                ButtonAction::Back,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child(Text::new("Retour"));
    });
}