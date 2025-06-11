use bevy::prelude::*;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect};
use crate::services::ui::constants::NORMAL_BUTTON;
use crate::services::ui::game::{GameScreen, GameButtonAction, GameScreenState, ArenaPresentationUI};

/// Affiche l'écran de présentation de l'Arène.
/// Cet écran affiche les informations sur l'Arène sélectionnée, le Maître de l'Arène,
/// ses attaques, et un bouton pour continuer vers l'Arène.
/// # Arguments
/// - `commands`: Les commandes pour créer des entités dans Bevy.
/// - `game_state`: L'état du jeu contenant les informations nécessaires pour l'écran de présentation.
pub fn spawn_arena_presentation_screen(commands: &mut Commands, game_state: &GameScreenState) {
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
        BackgroundColor(Color::srgb(0.15, 0.15, 0.25)),
        GameScreen,
        ArenaPresentationUI,
    ))
    .with_children(|parent| {
        if let Some(selected_arena) = &game_state.selected_arena {
            parent.spawn(Text::new(format!("Présentation de l'Arène: {}", selected_arena)));
        } else {
            parent.spawn(Text::new("Présentation de l'Arène"));
        }

        if let Some(master_name) = &game_state.master_name {
            parent.spawn(Text::new(format!("Maître de l'Arène: {}", master_name)));
            parent.spawn(Text::new(game_state.master_dialogs.join("\n")));
        }
        
        if let Some(master_style) = &game_state.master_style {
            parent.spawn(Text::new(format!("Style: {}", master_style)));
        }

        if let Some(master_badge) = &game_state.master_badge {
            parent.spawn(Text::new(format!("Badge: {}", master_badge)));
        }

        parent.spawn(Text::new("Attaques:"));
        for attack in &game_state.master_attacks {
            parent.spawn(Text::new(format!("- {}", attack)));
        }

        // Bouton pour continuer vers l'Arène
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
                GameButtonAction::EncounterBouncer,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child(Text::new("Continuer"));
    });
}