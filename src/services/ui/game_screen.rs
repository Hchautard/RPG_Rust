use bevy::prelude::*;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect};
use crate::services::ui::constants::{ButtonAction, NORMAL_BUTTON};
use crate::models::caracter::bouncer::Bouncer;
use crate::models::arena::Arena;
// NOTE: Il faut ajouter Clone à Arena dans arena.rs
use crate::services::json_loader::JsonLoader;

#[derive(Component)]
pub struct GameScreen;

// Nouveaux composants pour identifier les écrans
#[derive(Component)]
pub struct BouncerQuestionUI;

#[derive(Component)]
pub struct ArenaUI;

// Ressource pour stocker l'état du jeu
#[derive(Resource, Default)]
pub struct GameScreenState {
    pub current_screen: GameScreenType,
    pub current_question: String,
    pub answer_options: Vec<String>,
    pub correct_answer: String,
    pub available_arenas: Vec<(String, String)>, // (nom, thème)
    pub selected_arena: Option<String>,
}

#[derive(Default, PartialEq)]
pub enum GameScreenType {
    #[default]
    Main,
    ArenaSelection,
    BouncerQuestion,
    Arena,
}

// Nouvelles actions pour le jeu
#[derive(Component, Clone)]
pub enum GameButtonAction {
    SelectArena,
    ChooseArena(usize),
    EncounterBouncer,
    AnswerQuestion(usize),
    BackToMainGame,
    BackToArenaSelection,
}

pub fn setup_game(mut commands: Commands, mut game_state: ResMut<GameScreenState>) {
    // Initialiser l'état
    game_state.current_screen = GameScreenType::Main;
    
    // Charger les arènes depuis le JSON
    if let Ok(arenas) = JsonLoader::loadJsonArena("assets/caracters/arena.json") {
        game_state.available_arenas = arenas.iter()
            .map(|arena| (arena.name.clone(), arena.theme.clone()))
            .collect();
    }
    
    // Charger les données du bouncer
    if let Ok(bouncers) = JsonLoader::loadJsonBouncers("assets/caracters/pnj/bouncer.json") {
        if let Some(bouncer) = bouncers.first() {
            let question = bouncer.enigmas.first().unwrap_or(&"Question par défaut".to_string()).clone();
            let options = vec![
                "L'homme".to_string(),
                "Un animal".to_string(), 
                "Une machine".to_string(),
                "Le temps".to_string(),
            ];
            
            game_state.current_question = question;
            game_state.answer_options = options;
            game_state.correct_answer = "L'homme".to_string();
        }
    }

    spawn_main_game_screen(&mut commands);
}

fn spawn_main_game_screen(commands: &mut Commands) {
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
        GameScreen,
    ))
    .with_children(|parent| {
        parent.spawn(Text::new("Game Screen"));
        
        // Bouton pour sélectionner une arène
        parent
            .spawn((
                Button,
                Node {
                    width: Val::Px(250.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
                GameButtonAction::SelectArena,
            ))
            .with_child(Text::new("Sélectionner une Arène"));

        // Bouton retour
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
                ButtonAction::Back,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child(Text::new("Retour"));
    });
}

fn spawn_arena_selection_screen(commands: &mut Commands, game_state: &GameScreenState) {
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
        BackgroundColor(Color::srgb(0.1, 0.2, 0.3)),
        GameScreen,
    ))
    .with_children(|parent| {
        // Titre
        parent.spawn(Text::new("🏟️ Choisissez votre Arène 🏟️"));
        
        // Description
        parent.spawn(Text::new("Sélectionnez l'arène dans laquelle vous souhaitez vous battre"));
        
        // Container pour les arènes
        parent.spawn(
            Node {
                width: Val::Percent(80.0),
                margin: UiRect::vertical(Val::Px(20.0)),
                display: Display::Flex,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                ..Default::default()
            }
        )
        .with_children(|arenas_container| {
            // Créer un bouton pour chaque arène
            for (index, (arena_name, arena_theme)) in game_state.available_arenas.iter().enumerate() {
                arenas_container
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(200.0),
                            height: Val::Px(120.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        BorderColor(Color::WHITE),
                        BorderRadius::MAX,
                        BackgroundColor(NORMAL_BUTTON),
                        GameButtonAction::ChooseArena(index),
                    ))
                    .with_children(|button| {
                        button.spawn(Text::new(arena_name.clone()));
                        button.spawn(Text::new(format!("Thème: {}", arena_theme)));
                    });
            }
        });
        
        // Bouton retour
        parent
            .spawn((
                Button,
                Node {
                    width: Val::Px(200.0),
                    height: Val::Px(50.0),
                    margin: UiRect::all(Val::Px(20.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                GameButtonAction::BackToMainGame,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child(Text::new("Retour"));
    });
}

fn spawn_bouncer_question_screen(commands: &mut Commands, game_state: &GameScreenState) {
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
        GameScreen,
        BouncerQuestionUI,
    ))
    .with_children(|parent| {
        // Titre avec l'arène sélectionnée
        if let Some(selected_arena) = &game_state.selected_arena {
            parent.spawn(Text::new(format!("Arène: {} - Bouncer: Ragnar", selected_arena)));
        } else {
            parent.spawn(Text::new("Bouncer: Ragnar"));
        }
        
        // Question
        parent.spawn(Text::new(format!("Question: {}", game_state.current_question)));
        
        // 4 boutons de réponse
        for (index, option) in game_state.answer_options.iter().enumerate() {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(300.0),
                        height: Val::Px(50.0),
                        margin: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                    GameButtonAction::AnswerQuestion(index),
                ))
                .with_child(Text::new(format!("{}. {}", index + 1, option)));
        }
        
        // Bouton retour vers la sélection d'arène
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
                GameButtonAction::BackToArenaSelection,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child(Text::new("Retour"));
    });
}

fn spawn_arena_screen(commands: &mut Commands, game_state: &GameScreenState) {
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
        GameScreen,
        ArenaUI,
    ))
    .with_children(|parent| {
        // Affichage du nom de l'arène
        if let Some(selected_arena) = &game_state.selected_arena {
            parent.spawn(Text::new(format!("🏆 ARÈNE: {} 🏆", selected_arena)));
        } else {
            parent.spawn(Text::new("🏆 ARÈNE 🏆"));
        }
        
        // Bouton retour
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
                ButtonAction::Back,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child(Text::new("Retour"));
    });
}

// Système pour gérer les actions du jeu
pub fn handle_game_button_actions(
    mut interaction_query: Query<(
        &Interaction,
        &GameButtonAction,
        &mut BackgroundColor,
    ), (Changed<Interaction>, With<Button>)>,
    mut commands: Commands,
    mut game_state: ResMut<GameScreenState>,
    game_entities: Query<Entity, With<GameScreen>>,
) {
    for (interaction, action, mut background_color) in interaction_query.iter_mut() {
                    match *interaction {
            Interaction::Pressed => {
                match action {
                    GameButtonAction::SelectArena => {
                        // Nettoyer l'écran actuel
                        for entity in game_entities.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                        
                        // Aller à la sélection d'arène
                        game_state.current_screen = GameScreenType::ArenaSelection;
                        spawn_arena_selection_screen(&mut commands, &game_state);
                    }
                    GameButtonAction::ChooseArena(arena_index) => {
                        // Sauvegarder l'arène sélectionnée
                        if let Some((arena_name, _)) = game_state.available_arenas.get(*arena_index) {
                            game_state.selected_arena = Some(arena_name.clone());
                        }
                        
                        // Nettoyer l'écran actuel
                        for entity in game_entities.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                        
                        // Aller à la question du bouncer
                        game_state.current_screen = GameScreenType::BouncerQuestion;
                        spawn_bouncer_question_screen(&mut commands, &game_state);
                    }
                    GameButtonAction::EncounterBouncer => {
                        // Nettoyer l'écran actuel
                        for entity in game_entities.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                        
                        // Changer l'état et afficher l'écran de question
                        game_state.current_screen = GameScreenType::BouncerQuestion;
                        spawn_bouncer_question_screen(&mut commands, &game_state);
                    }
                    GameButtonAction::AnswerQuestion(answer_index) => {
                        let selected_answer = &game_state.answer_options[*answer_index];
                        
                        // Nettoyer l'écran actuel
                        for entity in game_entities.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                        
                        if *selected_answer == game_state.correct_answer {
                            // Bonne réponse -> Arène
                            game_state.current_screen = GameScreenType::Arena;
                            spawn_arena_screen(&mut commands, &game_state);
                        } else {
                            // Mauvaise réponse -> Retour au jeu principal
                            game_state.current_screen = GameScreenType::Main;
                            spawn_main_game_screen(&mut commands);
                        }
                    }
                    GameButtonAction::BackToArenaSelection => {
                        // Nettoyer l'écran actuel
                        for entity in game_entities.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                        
                        // Retour à la sélection d'arène
                        game_state.current_screen = GameScreenType::ArenaSelection;
                        spawn_arena_selection_screen(&mut commands, &game_state);
                    }
                    GameButtonAction::BackToMainGame => {
                        // Nettoyer l'écran actuel
                        for entity in game_entities.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                        
                        // Retour à l'écran principal du jeu
                        game_state.current_screen = GameScreenType::Main;
                        spawn_main_game_screen(&mut commands);
                    }
                }
                
                *background_color = Color::srgb(0.3, 0.3, 0.5).into();
            }
            Interaction::Hovered => {
                *background_color = Color::srgb(0.25, 0.25, 0.35).into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn despawn_game(mut commands: Commands, query: Query<Entity, With<GameScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}