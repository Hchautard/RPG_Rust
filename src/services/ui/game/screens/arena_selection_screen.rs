use bevy::prelude::*;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect, FlexWrap};
use crate::services::ui::constants::NORMAL_BUTTON;
use crate::services::ui::game::{GameScreen, GameButtonAction, GameScreenState};

/// Affiche l'écran de sélection de l'arène.
/// Cet écran permet de choisir une arène pour commencer un combat.
///
/// # Arguments
/// - `commands`: Les commandes pour créer des entités dans Bevy.
/// - `game_state`: L'état du jeu contenant les informations nécessaires pour l'écran de sélection.
pub fn spawn_arena_selection_screen(commands: &mut Commands, game_state: &GameScreenState) {
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
        BackgroundColor(Color::srgb(0.1, 0.2, 0.3)),
        GameScreen,
    ))
    .with_children(|parent| {
        parent.spawn(Text::new("Choisissez votre Arene"));
        
        // Message d'erreur si mauvaise réponse au bouncer
        if game_state.wrong_answer_message {
            parent.spawn((
                Text::new("Mauvaise réponse ! Vous avez été expulsé de l'entrée."),
                Node {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                },
            ));
        }
        
        // Description
        parent.spawn(Text::new("Sélectionnez l'arène dans laquelle vous souhaitez vous battre"));
        
        // Container pour les arènes
        parent.spawn(
            Node {
                width: Val::Percent(80.0),
                margin: UiRect::vertical(Val::Px(20.0)),
                display: Display::Flex,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                ..Default::default()
            }
        )
        .with_children(|arenas_container| {
            // Créer un bouton pour chaque arène
            for (index, (arena_name, arena_theme)) in game_state.available_arenas.iter().enumerate() {
                arenas_container
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(120.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        BorderColor(Color::WHITE),
                        BorderRadius::MAX,
                        BackgroundColor(NORMAL_BUTTON),
                        GameButtonAction::ChooseArena(index),
                    ))
                    .with_children(|button| {
                        button.spawn(Text::new(arena_name.clone()));
                        button.spawn(Text::new(format!("Thème: {}", arena_theme)));
                    });
            }
        });
        
        // Bouton retour
        parent
            .spawn((
                Button,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                GameButtonAction::BackToMainGame,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child(Text::new("Retour"));
    });
}