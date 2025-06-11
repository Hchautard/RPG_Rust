use crate::services::ui::constants::ButtonAction;
use bevy::prelude::*;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, Val};

/// Marqueur de composant pour identifier les entités du menu principal
#[derive(Component)]
pub struct MainMenu;

/// On crée l'interface du menu principal avec les boutons de navigation
/// # Arguments
/// - `commands`: Les commandes pour créer des entités dans Bevy.
pub fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // On crée les boutons du menu principal

            // Spawn buttons
            for (label, action) in [
                ("Nouvelle Partie", ButtonAction::NewGame),
                ("Charger Partie", ButtonAction::LoadGame),
                ("Voir les Aptitudes", ButtonAction::ShowAptitudes),
                ("Quitter", ButtonAction::Quit),
            ] {
                parent
                    .spawn((
                        Button {
                            ..Default::default()
                        },
                        action,
                    ))
                    .with_children(|button| {
                        // Texte du bouton

                        button.spawn(Text::from(label));
                    });
            }
        });
}

// On ,ettoie le menu principal en supprimant toutes les entités associées
pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
