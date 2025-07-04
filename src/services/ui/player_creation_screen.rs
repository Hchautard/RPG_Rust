use bevy::prelude::*;
use crate::services::ui::constants::{AppState, ButtonAction, NORMAL_BUTTON, BLACK, GREEN};
use crate::services::ui::player_slot_screen::SelectedPlayerSlot;
use crate::models::aptitude::Aptitude;
use crate::models::caracter::player::Player;
use crate::models::badge::Badge;
use crate::models::ingredient::Ingredient;
use bevy::ecs::system::ParamSet;
use crate::services::json_loader::JsonLoader;

/// Composant pour marquer les entites de l'ecran de creation de personnage
#[derive(Component)]
pub struct PlayerCreationScreen;

/// Ressource pour stocker les donnees temporaires de creation de personnage
#[derive(Resource)]
pub struct PlayerCreationData {
    pub name: String,
    pub style: String,
    pub selected_badge_index: Option<usize>,
    pub selected_aptitudes: Vec<usize>,
}

impl Default for PlayerCreationData {
    fn default() -> Self {
        Self {
            name: String::new(),
            style: String::new(),
            selected_badge_index: None,
            selected_aptitudes: Vec::new(),
        }
    }
}

/// Composant pour les champs de texte
#[derive(Component)]
pub enum TextInput {
    Name,
    Style,
}

/// Composant pour les choix de badges
#[derive(Component)]
pub struct BadgeChoice(pub usize);

/// Composant pour les choix d'aptitudes
#[derive(Component)]
pub struct AptitudeChoice(pub usize);

/// Systemes de creation de personnage
pub struct PlayerCreationPlugin;

impl Plugin for PlayerCreationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerCreationData>()
            .add_systems(OnEnter(AppState::PlayerCreation), setup_player_creation_screen)
            .add_systems(OnExit(AppState::PlayerCreation), despawn_player_creation_screen)
            .add_systems(Update, handle_text_input)
            .add_systems(Update, handle_badge_selection)
            .add_systems(Update, handle_aptitude_selection)
            .add_systems(Update, handle_creation_confirmation);
    }
}

pub fn setup_player_creation_screen(
    mut commands: Commands,
    selected_slot: Res<SelectedPlayerSlot>,
    mut creation_data: ResMut<PlayerCreationData>,
) {
    // Reinitialise les donnees de creation
    *creation_data = PlayerCreationData::default();

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
            PlayerCreationScreen,
        ))
        .with_children(|parent| {
            // Titre
            parent.spawn(Text::new(format!(
                "Creation de personnage - Slot {}",
                selected_slot.slot.unwrap_or(0) + 1
            )));

            // Formulaire principal
            parent
                .spawn(Node {
                    width: Val::Percent(80.0),
                    height: Val::Percent(80.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    margin: UiRect::all(Val::Px(20.0)),
                    ..Default::default()
                })
                .with_children(|form| {
                    // Champ Nom
                    form.spawn(Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(10.0)),
                        ..Default::default()
                    })
                    .with_children(|row| {
                        row.spawn(Text::new("Nom du personnage:"));
                        row.spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(30.0),
                                margin: UiRect::left(Val::Px(10.0)),
                                display: Display::Flex,
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                padding: UiRect::horizontal(Val::Px(10.0)),
                                ..Default::default()
                            },
                            BorderColor(BLACK),
                            TextInput::Name,
                        ))
                        .with_children(|input| {
                            input.spawn(Text::new("Cliquez pour saisir"));
                        });
                    });

                    // Selection de badge
                    form.spawn(Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        margin: UiRect::vertical(Val::Px(10.0)),
                        ..Default::default()
                    })
                    .with_children(|section| {
                        section.spawn(Text::new("Choisissez un badge:"));
                        
                        section.spawn(Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::FlexStart,
                            margin: UiRect::top(Val::Px(10.0)),
                            ..Default::default()
                        })
                        .with_children(|badges_container| {
                            // Exemple de badges
                            for i in 0..3 {
                                badges_container.spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(100.0),
                                        height: Val::Px(100.0),
                                        margin: UiRect::horizontal(Val::Px(10.0)),
                                        display: Display::Flex,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    BorderColor(BLACK),
                                    BackgroundColor(NORMAL_BUTTON),
                                    BadgeChoice(i),
                                ))
                                .with_children(|badge| {
                                    badge.spawn(Text::new(format!("Badge {}", i + 1)));
                                });
                            }
                        });
                    });

                    // Selection d'aptitudes
                    form.spawn(Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        margin: UiRect::vertical(Val::Px(10.0)),
                        ..Default::default()
                    })
                    .with_children(|section| {
                        section.spawn(Text::new("Choisissez des aptitudes:"));
                        
                        section.spawn(Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            flex_wrap: FlexWrap::Wrap,
                            justify_content: JustifyContent::FlexStart,
                            margin: UiRect::top(Val::Px(10.0)),
                            ..Default::default()
                        })
                        .with_children(|aptitudes_container| {
                            // Exemple d'aptitudes
                            for i in 0..6 {
                                aptitudes_container.spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(120.0),
                                        height: Val::Px(50.0),
                                        margin: UiRect::all(Val::Px(5.0)),
                                        display: Display::Flex,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    BorderColor(BLACK),
                                    BackgroundColor(NORMAL_BUTTON),
                                    AptitudeChoice(i),
                                ))
                                .with_children(|aptitude| {
                                    aptitude.spawn(Text::new(format!("Aptitude {}", i + 1)));
                                });
                            }
                        });
                    });

                    // Boutons de confirmation et retour
                    form.spawn(Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..Default::default()
                    })
                    .with_children(|button_row| {
                        // Bouton Confirmer
                        button_row.spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(50.0),
                                margin: UiRect::horizontal(Val::Px(10.0)),
                                display: Display::Flex,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            BorderColor(BLACK),
                            BackgroundColor(NORMAL_BUTTON),
                            ButtonAction::CreatePlayer,
                        ))
                        .with_children(|button| {
                            button.spawn(Text::new("Creer personnage"));
                        });

                        // Bouton Retour
                        button_row.spawn((
                            Button,
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(50.0),
                                margin: UiRect::horizontal(Val::Px(10.0)),
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
                });
        });
}

pub fn despawn_player_creation_screen(
    mut commands: Commands,
    query: Query<Entity, With<PlayerCreationScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Systeme de gestion des saisies texte ( avec noms aleatoires)
pub fn handle_text_input(
    mut interaction_query: Query<(&Interaction, &TextInput, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
    mut creation_data: ResMut<PlayerCreationData>,
) {
    for (interaction, input_type, children) in interaction_query.iter_mut() {
        if let Interaction::Pressed = *interaction {
            // Utilise un nom aleatoire au lieu d'une saisie reelle
            match *input_type {  
                TextInput::Name => {
                    // Liste de noms de heros aleatoires
                    let hero_names = [
                        "Aragorn", "Legolas", "Gandalf", "Frodo", "Gimli",
                        "Thorin", "Elendil", "Boromir", "Faramir", "Eomer",
                        "Drizzt", "Artemis", "Conan", "Geralt", "Ciri",
                        "Galadriel", "Arwen", "Eowyn", "Luthien", "Tauriel"
                    ];
                    
                    // Choisir un nom aleatoire
                    let random_index = (std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() % hero_names.len() as u128) as usize;
                    
                    creation_data.name = hero_names[random_index].to_string();
                    
                    if let Ok(mut text) = text_query.get_mut(*children.first().unwrap()) {
                        *text = Text::new(creation_data.name.clone());
                    }
                },
                TextInput::Style => {
                    // Styles de combat aleatoires
                    let combat_styles = [
                        "Offensif", "Defensif", "Equilibre", "Furtif", "Agressif",
                        "Strategique", "Acrobatique", "Magique", "Technique", "Berserker"
                    ];
                    
                    // Choisir un style aleatoire
                    let random_index = (std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() % combat_styles.len() as u128) as usize;
                    
                    creation_data.style = combat_styles[random_index].to_string();
                    
                    if let Ok(mut text) = text_query.get_mut(*children.first().unwrap()) {
                        *text = Text::new(creation_data.style.clone());
                    }
                }
            }
        }
    }
}

// Systeme de gestion de la selection de badge
pub fn handle_badge_selection(
    mut params: ParamSet<(
        Query<(&Interaction, &BadgeChoice), (Changed<Interaction>, With<Button>)>,
        Query<(&BadgeChoice, &mut BackgroundColor), With<Button>>
    )>,
    mut creation_data: ResMut<PlayerCreationData>,
) {
    // Trouve quel badge a ete clique
    let mut clicked_index = None;
    
    for (interaction, badge_choice) in params.p0().iter() {
        if let Interaction::Pressed = *interaction {
            clicked_index = Some(badge_choice.0);
            break;
        }
    }
    
    // Si un badge a ete clique
    if let Some(index) = clicked_index {
        // Si on clique sur un badge dejà selectionne, on le deselectionne
        if creation_data.selected_badge_index == Some(index) {
            creation_data.selected_badge_index = None;
            
            // Mise à jour des couleurs des boutons
            for (choice, mut bg_color) in params.p1().iter_mut() {
                if choice.0 == index {
                    *bg_color = BackgroundColor(NORMAL_BUTTON);
                }
            }
        } else {
            // Remplace la selection actuelle par le nouveau badge
            let previous_selection = creation_data.selected_badge_index;
            creation_data.selected_badge_index = Some(index);
            
            // Mise à jour des couleurs de tous les boutons
            for (choice, mut bg_color) in params.p1().iter_mut() {
                if choice.0 == index {
                    *bg_color = BackgroundColor(GREEN);
                } else if previous_selection == Some(choice.0) {
                    // Reinitialise le bouton precedemment selectionne
                    *bg_color = BackgroundColor(NORMAL_BUTTON);
                }
            }
        }
    }
}

// Systeme de gestion de la selection d'aptitudes
pub fn handle_aptitude_selection(
    mut params: ParamSet<(
        Query<(&Interaction, &AptitudeChoice), (Changed<Interaction>, With<Button>)>,
        Query<(&AptitudeChoice, &mut BackgroundColor), With<Button>>
    )>,
    mut creation_data: ResMut<PlayerCreationData>,
) {
    let mut clicked_index = None;
    
    for (interaction, aptitude_choice) in params.p0().iter() {
        if let Interaction::Pressed = *interaction {
            clicked_index = Some(aptitude_choice.0);
            break;
        }
    }
    
    // Si une aptitude a ete cliquee
    if let Some(index) = clicked_index {
        // Si on clique sur une aptitude dejà selectionnee, on la deselectionne
        if creation_data.selected_aptitudes.contains(&index) {
            creation_data.selected_aptitudes.retain(|&i| i != index);
            
            // Mise à jour des couleurs des boutons
            for (choice, mut bg_color) in params.p1().iter_mut() {
                if choice.0 == index {
                    *bg_color = BackgroundColor(NORMAL_BUTTON);
                }
            }
        } else {
            // Remplace la selection actuelle par la nouvelle aptitude
            creation_data.selected_aptitudes.clear();
            creation_data.selected_aptitudes.push(index);
            
            // Mise à jour des couleurs de tous les boutons
            for (choice, mut bg_color) in params.p1().iter_mut() {
                if choice.0 == index {
                    *bg_color = BackgroundColor(GREEN);
                } else {
                    *bg_color = BackgroundColor(NORMAL_BUTTON);
                }
            }
        }
    }
}

// Systeme de gestion de la confirmation de creation
pub fn handle_creation_confirmation(
    interaction_query: Query<(&Interaction, &ButtonAction), (Changed<Interaction>, With<Button>)>,
    creation_data: Res<PlayerCreationData>,
    selected_slot: Res<SelectedPlayerSlot>,
    mut next_state: ResMut<NextState<AppState>>,
) {

    JsonLoader::ensure_save_directory();

    for (interaction, action) in interaction_query.iter() {
        if let Interaction::Pressed = *interaction {
            if let ButtonAction::CreatePlayer = action {
                // Verifie que les informations necessaires sont renseignees
                if !creation_data.name.is_empty() && 
                   creation_data.selected_badge_index.is_some() && !creation_data.selected_aptitudes.is_empty() {
                    
                    // Creer le joueur
                    let player = create_player(&creation_data);
                    
                    // Determine le chemin du fichier base sur le slot
                    let slot_index = selected_slot.slot.unwrap_or(0);
                    let file_path = format!("save/player_slot_{}.json", slot_index + 1);
                    
                    // Sauvegarde le joueur dans un fichier JSON
                    match JsonLoader::save_player_to_json(&file_path, &player) {
                        Ok(_) => {
                            println!("Joueur sauvegarde dans {}", file_path);
                            // Revenir à l'ecran de selection de slot ou au menu principal
                            next_state.set(AppState::PlayerSlot);
                        },
                        Err(e) => {
                            println!("Erreur lors de la sauvegarde du joueur: {}", e);
                        }
                    }
                } else {
                    println!("Veuillez completer toutes les informations avant de creer le personnage.");
                }
            }
        }
    }
}

// Fonction pour creer un nouveau joueur à partir des donnees de creation
pub fn create_player(creation_data: &PlayerCreationData) -> Player {
    let badge = Badge {
        name: format!("Badge {}", creation_data.selected_badge_index.unwrap_or(0) + 1),
        features: vec!["Feature 1".to_string(), "Feature 2".to_string()],
    };
    
    let aptitudes = creation_data
        .selected_aptitudes
        .iter()
        .map(|&index| Aptitude {
            name: format!("Aptitude {}", index + 1),
            description: "Description de l'aptitude".to_string(),
            pp: 10,
            power: 5.0,
        })
        .collect();
    
    Player::new(
        &creation_data.name,
        &creation_data.style,
        badge,
        Vec::<Ingredient>::new(),
        aptitudes,
    )
}