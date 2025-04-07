use bevy::prelude::*;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection};
use crate::services::ui::constants::{ButtonAction, NORMAL_BUTTON};

#[derive(Component)]
pub struct MainMenu;

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
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
                    button.spawn(Text::from(label));
                });
        }
    });
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}