use bevy::{
    prelude::*,
    ui::{widget::{Button, ImageNode, Text}, AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val},
};

use super::constants::{AppState, ButtonAction, Level, LevelList};

/// Marqueur pour l'UI de l'écran de niveaux
#[derive(Component)]
pub struct LevelsScreenUI;

/// Plugin pour gérer l'écran des levels
pub struct LevelsScreenPlugin;

impl Plugin for LevelsScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Levels), setup_levels_screen)
           .add_systems(OnExit(AppState::LevelsScreen), cleanup_levels_screen);
    }
}

/// Création de l'écran des levels
pub fn setup_levels_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut level_list: ResMut<LevelList>,
) {
    level_list.levels = load_levels(&asset_server);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
        LevelsScreenUI,
    ))
    .with_children(|parent| {
        for level in &level_list.levels {
            let button_action = if level.is_locked {
                ButtonAction::Locked
            } else {
                ButtonAction::SelectLevel(level.clone())
            };

            parent.spawn((
                Button::default(),
                button_action,
            ))
            .with_children(|button| {
                button.spawn(ImageNode {
                    image: level.image.clone(),
                    ..Default::default()
                });
                button.spawn(Text::from(format!("{} - {}", level.name, level.boss_name)));
            });
        }
    });
}

/// Suppression de l'UI quand on quitte l'écran
fn cleanup_levels_screen(
    mut commands: Commands,
    query: Query<Entity, With<LevelsScreenUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

/// Exemple de chargement des levels
fn load_levels(asset_server: &Res<AssetServer>) -> Vec<Level> {
    vec![
        Level {
            index: 0,
            name: "Niveau 1".to_string(),
            image: asset_server.load("images/OIP.png"),
            is_locked: false,
            boss_name: "Boss 1".to_string(),
        },
        Level {
            index: 1,
            name: "Niveau 2".to_string(),
            image: asset_server.load("images/boss2.png"),
            is_locked: true,
            boss_name: "Boss 2".to_string(),
        },
    ]
}
