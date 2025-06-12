use crate::services::ui::constants::{AppState, ButtonAction, BLACK, NORMAL_BUTTON, WHITE};
use crate::services::ui::player_slot_screen::SelectedPlayerSlot;
use bevy::prelude::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// Composant pour marquer les entites de l'ecran de demarrage
#[derive(Component)]
pub struct StartScreen;

// Ressource pour stocker le texte charge
#[derive(Resource)]
pub struct StartScreenText {
    pub content: String,
}

impl Default for StartScreenText {
    fn default() -> Self {
        Self {
            content: "Aucun contenu charge.".to_string(),
        }
    }
}

// Plugin pour l'ecran de demarrage
pub struct StartScreenPlugin;

impl Plugin for StartScreenPlugin {
    // On configure les systèmes de l'écran de démarrage

    fn build(&self, app: &mut App) {
        app.init_resource::<StartScreenText>()
            .add_systems(OnEnter(AppState::StartScreen), setup_start_screen)
            .add_systems(OnExit(AppState::StartScreen), despawn_start_screen)
            .add_systems(OnEnter(AppState::StartScreen), load_save_content);
    }
}

// Systeme pour charger le contenu de la sauvegarde
pub fn load_save_content(
    mut start_text: ResMut<StartScreenText>,
    selected_slot: Res<SelectedPlayerSlot>,
) {
    if let Some(slot) = selected_slot.slot {
        // On construit le chemin du fichier de sauvegarde
        let file_path = format!("save/player_slot_{}.json", slot + 1);

        if Path::new(&file_path).exists() {
            match File::open(&file_path) {
                Ok(mut file) => {
                    let mut content = String::new();
                    if file.read_to_string(&mut content).is_ok() {
                        // On essaye de formater en JSON pour une meilleure lisibilite
                        match serde_json::from_str::<serde_json::Value>(&content) {
                            Ok(json) => {
                                start_text.content =
                                    serde_json::to_string_pretty(&json).unwrap_or(content);
                            }
                            Err(_) => {
                                // Si ce n'est pas du JSON, on affiche en brut
                                start_text.content = content;
                            }
                        }
                    } else {
                        start_text.content = "Erreur lors de la lecture du fichier.".to_string();
                    }
                }
                Err(_) => {
                    start_text.content =
                        "Impossible d'ouvrir le fichier de sauvegarde.".to_string();
                }
            }
        } else {
            start_text.content = "Le fichier de sauvegarde n'existe pas.".to_string();
        }
    } else {
        start_text.content = "Aucun slot selectionne.".to_string();
    }
}

// On crée l'interface de l'écran de démarrage avec le contenu de sauvegarde
pub fn setup_start_screen(mut commands: Commands, start_text: Res<StartScreenText>) {
    // Conteneur principal du plein écran
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            StartScreen,
        ))
        .with_children(|parent| {
            // Titre de l'écran

            parent.spawn(Text::new("Contenu de la sauvegarde"));

            // Zone d'affichage du contenu avec défilement

            parent
                .spawn((
                    Node {
                        width: Val::Percent(80.0),
                        height: Val::Percent(70.0),
                        margin: UiRect::vertical(Val::Px(20.0)),
                        padding: UiRect::all(Val::Px(10.0)),
                        display: Display::Flex,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::visible(),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    BorderColor(WHITE),
                ))
                .with_children(|text_area| {
                    // Affichage du contenu de la sauvegarde

                    text_area.spawn(Text::new(start_text.content.clone()));
                });

            // Bouton pour démarrer le jeu

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderColor(BLACK),
                    BackgroundColor(NORMAL_BUTTON),
                    ButtonAction::StartGame,
                ))
                .with_children(|button| {
                    button.spawn(Text::new("Demarrer le jeu"));
                });

            // Bouton de retour au menu précédent

            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderColor(BLACK),
                    BackgroundColor(NORMAL_BUTTON),
                    ButtonAction::Back,
                ))
                .with_children(|button| {
                    button.spawn(Text::new("Retour"));
                });
        });
}

// On nettoie l'écran en supprimant toutes les entités associées

pub fn despawn_start_screen(mut commands: Commands, query: Query<Entity, With<StartScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
