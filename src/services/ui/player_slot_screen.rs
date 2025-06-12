use crate::services::ui::constants::{
    AppState, ButtonAction, GameLoadContext, BLACK, BLUE, NORMAL_BUTTON, SELECTED_BUTTON,
};
use bevy::prelude::*;
use serde_json::Value;
use std::fs::File;
use std::path::Path;

// Composant pour marquer les entites de l'ecran de selection de slot
#[derive(Component)]
pub struct PlayerSlotScreen;

// Ressource pour stocker le slot de joueur selectionne
#[derive(Resource)]
pub struct SelectedPlayerSlot {
    pub slot: Option<usize>,
}

// Structure pour stocker les informations des slots
#[derive(Resource)]
pub struct SlotInfo {
    // Vecteur de 3 slots chaque élément contient le nom du joueur ou None si vide
    pub info: Vec<Option<String>>,
}

impl Default for SlotInfo {
    fn default() -> Self {
        Self {
            info: vec![None, None, None],
        }
    }
}

// Plugin pour gérer l'écran de sélection des slots de sauvegarde

pub struct PlayerSlotScreenPlugin;

impl Plugin for PlayerSlotScreenPlugin {
    // On configure les systèmes de gestion des slots

    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedPlayerSlot>()
            .init_resource::<SlotInfo>()
            .add_systems(Startup, load_player_slots)
            .add_systems(OnEnter(AppState::PlayerSlot), load_player_slots)
            .add_systems(OnEnter(AppState::PlayerSlot), setup_player_slot_screen)
            .add_systems(OnExit(AppState::PlayerSlot), despawn_player_slot_screen)
            .add_systems(Update, update_slot_selection)
            .add_systems(OnExit(AppState::PlayerCreation), load_player_slots);
    }
}

impl Default for SelectedPlayerSlot {
    fn default() -> Self {
        Self { slot: None }
    }
}

// On charge les informations des 3 slots de sauvegarde depuis les fichiers JSON

pub fn load_player_slots(mut slot_info: ResMut<SlotInfo>) {
    // On crée le dossier de sauvegarde si nécessaire

    if !Path::new("save").exists() {
        if let Err(e) = std::fs::create_dir_all("save") {
            println!("Erreur lors de la creation du dossier de sauvegarde: {}", e);
        }
    }

    // On réinitialise les informations de slot
    slot_info.info = vec![None, None, None];

    // On vérifie chaque slot (1, 2, 3)

    for i in 0..3 {
        let file_path = format!("save/player_slot_{}.json", i + 1);
        if Path::new(&file_path).exists() {
            // On tente de lire le fichier JSON pour extraire le nom du joueur
            match File::open(&file_path) {
                Ok(file) => {
                    match serde_json::from_reader::<_, Value>(file) {
                        Ok(json) => {
                            // On parcours les entrées pour trouver le nom du personnage

                            if let Some(obj) = json.as_object() {
                                for (_, player_data) in obj {
                                    // On accède à caracter.name
                                    if let Some(character) = player_data.get("caracter") {
                                        if let Some(name) =
                                            character.get("name").and_then(|n| n.as_str())
                                        {
                                            slot_info.info[i] = Some(name.to_string());
                                            break; // On prend le premier personnage trouve
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => println!(
                            "Erreur lors de la lecture du JSON pour le slot {}: {}",
                            i + 1,
                            e
                        ),
                    }
                }
                Err(e) => println!(
                    "Erreur lors de l'ouverture du fichier pour le slot {}: {}",
                    i + 1,
                    e
                ),
            }
        }
    }
}

// On crée l'interface de sélection des slots de sauvegarde

pub fn setup_player_slot_screen(
    mut commands: Commands,
    slot_info: Res<SlotInfo>,
    game_context: Res<GameLoadContext>,
) {
    // On adapte du texte selon le contexte (nouveau jeu ou chargement)
    let screen_title = if game_context.is_load_game {
        "Choisissez une sauvegarde a charger"
    } else {
        "Choisissez un slot pour la nouvelle partie"
    };

    let confirm_button_text = if game_context.is_load_game {
        "Charger"
    } else {
        "Confirmer"
    };
    // Conteneur principal de l'écran

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
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            PlayerSlotScreen,
        ))
        .with_children(|parent| {
            // Titre de l'écran

            parent.spawn(Text::new(screen_title));

            // Conteneur des 3 slots de sauvegarde

            parent
                .spawn((Node {
                    width: Val::Percent(80.0),
                    margin: UiRect::vertical(Val::Px(20.0)),
                    display: Display::Flex,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },))
                .with_children(|slot_container| {
                    // On crée les 3 boutons de slot

                    for i in 0..3 {
                        slot_container
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(150.0),
                                    height: Val::Px(150.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    display: Display::Flex,
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Column,
                                    ..Default::default()
                                },
                                BorderColor(BLACK),
                                BackgroundColor(NORMAL_BUTTON),
                                ButtonAction::SelectSlot(i),
                            ))
                            .with_children(|button| {
                                // Numéro du slot

                                button.spawn(Text::new(format!("Slot {}", i + 1)));

                                // on affiche le nom du personnage s'il existe sinon "Vide"
                                let slot_text = match &slot_info.info[i] {
                                    Some(name) => name.clone(),
                                    None => "Vide".to_string(),
                                };
                                button.spawn(Text::new(slot_text));
                            });
                    }
                });

            // Bouton de confirmation adapté au contexte
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(50.0),
                        margin: UiRect::vertical(Val::Px(20.0)),
                        display: Display::Flex,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderColor(BLACK),
                    BackgroundColor(NORMAL_BUTTON),
                    ButtonAction::ConfirmSlot,
                ))
                .with_children(|button| {
                    button.spawn(Text::new(confirm_button_text));
                });

            // Bouton de retour

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

// On met à jour l'apparence visuelle du slot sélectionné

pub fn update_slot_selection(
    selected_slot: Res<SelectedPlayerSlot>,
    mut slot_buttons: Query<(&ButtonAction, &mut BackgroundColor, &mut BorderColor)>,
) {
    for (action, mut bg_color, mut border_color) in slot_buttons.iter_mut() {
        if let ButtonAction::SelectSlot(slot_index) = action {
            if selected_slot.slot == Some(*slot_index) {
                // Slot sélectionné : couleurs spéciales

                *bg_color = BackgroundColor(SELECTED_BUTTON);
                *border_color = BorderColor(BLUE);
            } else {
                // Slot normal on utilise les couleurs par défaut

                *bg_color = BackgroundColor(NORMAL_BUTTON);
                *border_color = BorderColor(BLACK);
            }
        }
    }
}

// On nettoie l'écran en supprimant toutes les entités associées
pub fn despawn_player_slot_screen(
    mut commands: Commands,
    query: Query<Entity, With<PlayerSlotScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
