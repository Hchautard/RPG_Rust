use bevy::prelude::*;

use crate::{models::caracter::master::{self, Master}, services::json_loader::JsonLoader};

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
        BackgroundColor(Color::rgb(0.4, 0.4, 0.4)),
        LevelsScreenUI,
    ))
    .with_children(|parent| {
        // Main container - grid layout instead of scrollable container
        parent.spawn((
            Node {
                width: Val::Percent(90.0),
                height: Val::Percent(80.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_wrap: FlexWrap::Wrap,
                ..Default::default()
            },
            BackgroundColor(Color::rgb(0.4, 0.4, 0.4)),
        ))
        .with_children(|container| {
            for level in &level_list.levels {
                let button_action = if level.is_locked {
                    ButtonAction::Locked
                } else {
                    ButtonAction::SelectLevel(level.clone())
                };

                // Card for each level
                container.spawn((
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(400.0),
                        margin: UiRect::all(Val::Px(10.0)), 
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    BackgroundColor(Color::rgb(0.03, 0.03, 0.03)),
                    LevelsScreenUI,
                ))
                .with_children(|card| {
                    // Level image
                    card.spawn((
                        ImageNode { image: level.image.clone(), ..Default::default() },
                        Node {
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
                            ..Default::default()
                        },
                        BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
                    ));
                    card.spawn(Node {
                        width: Val::Px(0.0),
                        height: Val::Px(20.0),
                        ..Default::default()
                    });

                    card.spawn((
                        ImageNode { image: level.cocktail_image.clone(), ..Default::default() },
                        Node {
                            width: Val::Px(100.0),
                            height: Val::Px(100.0),
                            ..Default::default()
                        },
                        BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
                    ));
                    

                    // Level name
                    card.spawn(Text::from(format!("Level: {}", level.name)));
                    card.spawn(Text::from(format!("Master: {}", level.master.pnj.caracter.name)));

                    // Button for each level
                    card.spawn((
                        Button,
                        button_action.clone(),
                        BackgroundColor(Color::rgb(0.3, 0.3, 0.3)),
                    ))
                    .with_children(|b| {
                        if(level.is_locked) {
                            b.spawn(Text::from("Locked"));
                        } else {
                            b.spawn(Text::from("Select"));
                        }
                    });
                });
            }
        });
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
    let masters: Vec<Master> = JsonLoader::loadJsonMasters("assets/caracters/pnj/masters.json").unwrap();
    let mut levels: Vec<Level> = Vec::new();
    for master in masters {
        levels.push(Level {
            index: 0,
            name: "Niveau 1".to_string(),
            image: asset_server.load("images/OIP.png"),
            cocktail_image: asset_server.load("images/cocktail.png"),
            is_locked: true,
            master,
        });
    }
    if let Some(first_level) = levels.first_mut() {
        first_level.is_locked = false;
    }
    levels
}