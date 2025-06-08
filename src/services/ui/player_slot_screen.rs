use bevy::prelude::*;
use crate::services::ui::constants::{AppState, ButtonAction, GameLoadContext, NORMAL_BUTTON, SELECTED_BUTTON, WHITE, BLACK, BLUE};
use crate::services::json_loader::JsonLoader;
use std::fs::File;
use std::path::Path;
use serde_json::Value;

/// Composant pour marquer les entités de l'écran de sélection de slot
#[derive(Component)]
pub struct PlayerSlotScreen;

/// Ressource pour stocker le slot de joueur sélectionné
#[derive(Resource)]
pub struct SelectedPlayerSlot {
    pub slot: Option<usize>,
}

/// Structure pour stocker les informations des slots
#[derive(Resource)]
pub struct SlotInfo {
    pub info: Vec<Option<String>>,
}

impl Default for SlotInfo {
    fn default() -> Self {
        Self {
            info: vec![None, None, None],
        }
    }
}

/// Plugin pour l'écran de sélection de slot
pub struct PlayerSlotScreenPlugin;

impl Plugin for PlayerSlotScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectedPlayerSlot>()
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

/// Système pour charger les informations des slots
pub fn load_player_slots(mut slot_info: ResMut<SlotInfo>) {
    // S'assurer que le dossier de sauvegarde existe
    if !Path::new("save").exists() {
        if let Err(e) = std::fs::create_dir_all("save") {
            println!("Erreur lors de la création du dossier de sauvegarde: {}", e);
        }
    }
    
    // Réinitialiser les informations de slot
    slot_info.info = vec![None, None, None];
    
    // Vérifier chaque slot
    for i in 0..3 {
        let file_path = format!("save/player_slot_{}.json", i + 1);
        if Path::new(&file_path).exists() {
            // Tenter de lire le fichier JSON pour extraire le nom du joueur
            match File::open(&file_path) {
                Ok(file) => {
                    match serde_json::from_reader::<_, Value>(file) {
                        Ok(json) => {
                            // Parcourir les entrées (niveaux)
                            if let Some(obj) = json.as_object() {
                                for (_, player_data) in obj {
                                    // Accéder à caracter.name
                                    if let Some(character) = player_data.get("caracter") {
                                        if let Some(name) = character.get("name").and_then(|n| n.as_str()) {
                                            slot_info.info[i] = Some(name.to_string());
                                            break; // On prend le premier personnage trouvé
                                        }
                                    }
                                }
                            }
                        },
                        Err(e) => println!("Erreur lors de la lecture du JSON pour le slot {}: {}", i + 1, e),
                    }
                },
                Err(e) => println!("Erreur lors de l'ouverture du fichier pour le slot {}: {}", i + 1, e),
            }
        }
    }
}

/// Système pour initialiser l'écran de sélection de slot
pub fn setup_player_slot_screen(
    mut commands: Commands, 
    slot_info: Res<SlotInfo>,
    game_context: Res<GameLoadContext>
) {
    let screen_title = if game_context.is_load_game {
        "Choisissez une sauvegarde à charger"
    } else {
        "Choisissez un slot pour la nouvelle partie"
    };
    
    let confirm_button_text = if game_context.is_load_game {
        "Charger"
    } else {
        "Confirmer"
    };

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
        PlayerSlotScreen,
    ))
    .with_children(|parent| {
        parent.spawn(Text::new(screen_title));

        // Container pour les slots
        parent.spawn((
            Node {
                width: Val::Percent(80.0),
                margin: UiRect::vertical(Val::Px(20.0)),
                display: Display::Flex,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
        ))
        .with_children(|slot_container| {
            // Créer 3 slots de sauvegarde
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
                        button.spawn(Text::new(format!("Slot {}", i + 1)));
                        
                        // Afficher le nom du personnage s'il existe, sinon "Vide"
                        let slot_text = match &slot_info.info[i] {
                            Some(name) => name.clone(),
                            None => "Vide".to_string(),
                        };
                        button.spawn(Text::new(slot_text));
                    });
            }
        });

        // Bouton de confirmation avec texte adapté au contexte
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

        // Bouton retour
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

/// Système pour mettre à jour la sélection de slot
pub fn update_slot_selection(
    selected_slot: Res<SelectedPlayerSlot>,
    mut slot_buttons: Query<(&ButtonAction, &mut BackgroundColor, &mut BorderColor)>,
) {
    for (action, mut bg_color, mut border_color) in slot_buttons.iter_mut() {
        if let ButtonAction::SelectSlot(slot_index) = action {
            if selected_slot.slot == Some(*slot_index) {
                *bg_color = BackgroundColor(SELECTED_BUTTON);
                *border_color = BorderColor(BLUE);
            } else {
                *bg_color = BackgroundColor(NORMAL_BUTTON);
                *border_color = BorderColor(BLACK);
            }
        }
    }
}

/// Système pour supprimer l'écran de sélection de slot
pub fn despawn_player_slot_screen(mut commands: Commands, query: Query<Entity, With<PlayerSlotScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Modifier le système de boutons dans displayer_bevy.rs pour mettre à jour le slot sélectionné
pub fn extend_button_system_for_slot(
    interaction: &Interaction,
    action: &ButtonAction,
    selected_slot: &mut SelectedPlayerSlot,
) {
    if let Interaction::Pressed = interaction {
        if let ButtonAction::SelectSlot(slot_index) = action {
            selected_slot.slot = Some(*slot_index);
        }
    }
}