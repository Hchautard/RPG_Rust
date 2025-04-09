use bevy::prelude::*;
use bevy::ecs::query::With;
use bevy::ecs::component::Component;
use bevy::ecs::system::{Commands, Query, Res, ResMut};
use bevy::asset::{AssetServer, Handle};
use bevy::image::Image;
use bevy::ui::{widget::{Button, ImageNode, Text}, AlignItems, BackgroundColor, FlexDirection, JustifyContent, Node, Val};
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};

use crate::services::{combat_state::{CombatState, Turn}, ui::constants::{AppState, NORMAL_BUTTON}};
use super::constants::ButtonAction;

#[derive(Component)]
struct CombatScreen;

/// Plugin indépendant pour gérer l'écran de combat
pub struct CombatScreenPlugin;

impl Plugin for CombatScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Fight), setup_combat_screen)
            .add_systems(OnExit(AppState::Fight), cleanup_combat_screen)
            .add_systems(Update, combat_system);
    }
}


/// Setup de l'écran combat
fn setup_combat_screen(
    mut commands: Commands,
    combat: Res<CombatState>,
    asset_server: Res<AssetServer>,
) {
    let master_image: Handle<Image> = asset_server.load("images/OIP.png");

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        CombatScreen,
    ))
    .with_children(|parent| {
        parent.spawn(Text::from(format!("Combat contre : {}", combat.master.pnj.caracter.name)));
        parent.spawn(ImageNode { image: master_image.clone(), ..Default::default() });
        parent.spawn(Text::from(format!("HP Joueur : {}", combat.player.caracter.hp)));
        parent.spawn(Text::from(format!("HP Boss : {}", combat.master.pnj.caracter.hp)));

        parent.spawn((
            Button,
            ButtonAction::StartFight,
            BackgroundColor(NORMAL_BUTTON),
        ))
        .with_children(|b| { b.spawn(Text::from("Attaquer")); });

        parent.spawn((
            Button,
            ButtonAction::Back,
            BackgroundColor(NORMAL_BUTTON),
        ))
        .with_children(|b| { b.spawn(Text::from("Fuir")); });
    });
}

/// Nettoyage complet de l'écran combat
fn cleanup_combat_screen(
    mut commands: Commands,
    query: Query<Entity, With<CombatScreen>>,
) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

/// Système de combat (logique uniquement)
fn combat_system(
    mut next_state: ResMut<NextState<AppState>>,
) {
    /*if !combat.started || combat.finished {
        return;
    }

    if combat.player.caracter.hp <= 0 {
        println!("Le joueur a perdu !");
        combat.finished = true;
        next_state.set(AppState::MainMenu);
        return;
    }

    if combat.master.pnj.caracter.hp <= 0 {
        println!("Le joueur a gagné !");
        combat.finished = true;
        next_state.set(AppState::MainMenu);
        return;
    }

    match combat.turn {
        Turn::Player => {
            let damage = 10;
            combat.master.pnj.caracter.hp -= damage;
            println!("Le joueur inflige {} à {}", damage, combat.master.pnj.caracter.name);
            combat.turn = Turn::Master;
        }
        Turn::Master => {
            let damage = 8;
            combat.player.caracter.hp -= damage;
            println!("{} inflige {} au joueur", combat.master.pnj.caracter.name, damage);
            combat.turn = Turn::Player;
        }
    }*/
}
