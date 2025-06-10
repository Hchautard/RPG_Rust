use bevy::prelude::*;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect};
use crate::models::recipe::Recipe;
use crate::services::ui::constants::{ButtonAction, NORMAL_BUTTON};
use crate::models::caracter::bouncer::Bouncer;
use crate::models::arena::Arena;
use crate::services::json_loader::JsonLoader;
use std::collections::HashSet;

// Marqueur de composant pour l'écran de jeu principal
#[derive(Component)]
pub struct GameScreen;

// Marqueur de composant pour l'interface de question du videur
#[derive(Component)]
pub struct BouncerQuestionUI;
// Marqueur de composant pour l'interface d'arène
#[derive(Component)]
pub struct ArenaUI;
// État global de l'écran de jeu contenant toutes les données de session

#[derive(Resource, Default)]
pub struct GameScreenState {
    // Type d'écran actuellement affiché

    pub current_screen: GameScreenType,
     // Question posée par le videur

    pub current_question: String,
        // Options de réponse pour la question

    pub answer_options: Vec<String>,
        // Réponse correcte attendue

    pub correct_answer: String,
        // Liste des arènes disponibles

    pub available_arenas: Vec<(String, String)>,
        // Arène actuellement sélectionnée

    pub selected_arena: Option<String>,
    // Affichage du message d'erreur en cas de mauvaise répons
    pub wrong_answer_message: bool, 
        // Nom du maître de l'arène

    pub master_name: Option<String>,
        // Style de combat du maître

    pub master_style: Option<String>,
        // Liste des attaques du maître

    pub master_attacks: Vec<String>,
        // Dialogues du maître

    pub master_dialogs: Vec<String>,
        // Badge du maître

    pub master_badge: Option<String>,
        // Index de l'arène sélectionnée

    pub selected_arena_index: Option<usize>,
        // État du combat d'arène

    pub arena_combat_state: ArenaCombatState,
        // Points de vie du joueur

    pub player_hp: i32,
        // Points de vie du boss

    pub boss_hp: i32,
        // Affichage de l'écran d'introduction

    pub show_intro_screen: bool, 
        // Recette du maître à reproduire

    pub master_recipe: Option<Recipe>,
        // Attaque actuelle du boss

    pub current_boss_attack: Option<String>,
        // État de la création de cocktail en cours

    pub current_crafting: CurrentCocktailCrafting,
        // Affichage de la phase de craft

    pub show_crafting_phase: bool,
}

// Énumération des différents types d'écrans de jeu

#[derive(Default, PartialEq)]
pub enum GameScreenType {
    #[default]
    // Écran principal du jeu

    Main,
        // Sélection d'arène

    ArenaSelection,
        // Présentation de l'arène sélectionnée

    ArenaPresentation, 
        // Question du videur

    BouncerQuestion,
        // Combat dans l'arène

    Arena,
}

// Actions spécifiques aux boutons de l'écran de jeu

#[derive(Component, Clone)]
pub enum GameButtonAction {
        // Sélectionner une arène

    SelectArena,
        // Choisir une arène spécifique par index

    ChooseArena(usize),
        // Rencontrer le videur

    EncounterBouncer,
        // Répondre à une question (index de la réponse)

    AnswerQuestion(usize),
        // Retour au jeu principal

    BackToMainGame,
        // Retour à la sélection d'arène

    BackToArenaSelection,
        // Démarrer le combat

    StartCombat,
        // Sélectionner un ingrédient

    SelectIngredient(String),
        // Valider le cocktail créé

    ValidateCocktail,
        // Commencer le combat d'arène

    StartArenaCombat,
        // Retour depuis le combat

    BackToMainFromCombat,
        // Démarrer la phase finale de craft

    StartFinalCraft,
        // Sélectionner une instruction

    SelectInstruction(String),
        // Valider l'ordre des instructions

    ValidateInstructionOrder,

}

// Configuration initiale de l'écran de jeu

pub fn setup_game(mut commands: Commands, mut game_state: ResMut<GameScreenState>) {
    // On initialise l'etat par defaut
    game_state.current_screen = GameScreenType::Main;
    
    // On charges les arenes depuis le JSON
    if let Ok(arenas) = JsonLoader::loadJsonArena("assets/caracters/arena.json") {
        game_state.available_arenas = arenas.iter()
            .map(|arena| (arena.name.clone(), arena.theme.clone()))
            .collect();
    }
    
    // On charges les donnees du bouncer
    if let Ok(bouncers) = JsonLoader::loadJsonBouncers("assets/caracters/pnj/bouncer.json") {
        if let Some(bouncer) = bouncers.first() {
            let question = bouncer.enigmas.first().unwrap_or(&"Question par defaut".to_string()).clone();
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

// On crée l'écran principal du jeu avec les options de base

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
        parent.spawn(Text::new(""));
        
        // Bouton pour selectionner une arene
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
            .with_child(Text::new("Selectionner une Arene"));

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

//  On crée l'écran de sélection d'arène avec la liste des arènes disponibles

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
        parent.spawn(Text::new("Choisissez votre Arene"));
        
        // Message d'erreur si mauvaise reponse au bouncer
        if game_state.wrong_answer_message {
            parent.spawn((
                Text::new("Mauvaise reponse ! Vous avez ete expulse de l'entree."),
                Node {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..Default::default()
                },
            ));
        }
        
        // Description
        parent.spawn(Text::new("Selectionnez l'arene dans laquelle vous souhaitez vous battre"));
        
                // Conteneur des arènes disponibles

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
            // Creer un bouton pour chaque arene
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
                        button.spawn(Text::new(format!("Theme: {}", arena_theme)));
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

// On crée l'écran de question du videur avec les options de réponse
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
        // Titre avec l'arene selectionnee
        if let Some(selected_arena) = &game_state.selected_arena {
            parent.spawn(Text::new(format!("Arene: {} - Bouncer: Ragnar", selected_arena)));
        } else {
            parent.spawn(Text::new("Bouncer: Ragnar"));
        }
        
        // Question affichage
        parent.spawn(Text::new(format!("Question: {}", game_state.current_question)));
        
        // 4 boutons de reponse
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
        
        // Bouton retour vers la selection d'arene
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

// Systeme pour gerer les actions du jeu
pub fn handle_game_button_actions(
    mut interaction_query: Query<(
        &Interaction,
        &GameButtonAction,
        &mut BackgroundColor,
    ), (Changed<Interaction>, With<Button>)>,
    mut commands: Commands,
    mut game_state: ResMut<GameScreenState>,
    gameentities: Query<Entity, With<GameScreen>>,
    arena_ui_query: Query<Entity, With<ArenaUI>>,
) {
    for (interaction, action, mut background_color) in interaction_query.iter_mut() {
                    match *interaction {
            // Gestion des clics de boutons

            Interaction::Pressed => {
                match action {
                    GameButtonAction::SelectArena => {
                                        // On reinitialise le message d'erreur
                                        game_state.wrong_answer_message = false;
                        
                                        // On nettoie l'ecran actuel
                                        for entity in gameentities.iter() {
                                            commands.entity(entity).despawn_recursive();
                                        }
                        
                                        // pour aller à la selection d'arene
                                        game_state.current_screen = GameScreenType::ArenaSelection;
                                        spawn_arena_selection_screen(&mut commands, &game_state);
                                    }
                    GameButtonAction::ChooseArena(arena_index) => {
                                        game_state.wrong_answer_message = false;

                                        // Sauvegarde de l'arene selectionnee
                                        if let Some((arena_name, _)) = game_state.available_arenas.get(*arena_index) {
                                            game_state.selected_arena = Some(arena_name.clone());
                                            game_state.selected_arena_index = Some(*arena_index);
                                        }

                                        // nettoyage de l'ecran actuel
                                        for entity in gameentities.iter() {
                                            commands.entity(entity).despawn_recursive();
                                        }

                                        // pour aller à la question du bouncer
                                        game_state.current_screen = GameScreenType::BouncerQuestion;
                                        spawn_bouncer_question_screen(&mut commands, &game_state);
                                    }
                    GameButtonAction::EncounterBouncer => {
                    // netoyage de l'ecran actuel
                    for entity in gameentities.iter() {
                        commands.entity(entity).despawn_recursive();
                    }

                    // Change l'etat et affiche l'ecran de combat maintenant
                    game_state.current_screen = GameScreenType::Arena;
                    game_state.arena_combat_state = ArenaCombatState::Start;
                    game_state.player_hp = 100;
                    game_state.boss_hp = 100;
                    game_state.current_crafting = CurrentCocktailCrafting::default();

                    game_state.show_intro_screen = true;
                        spawn_arena_combat_screen(&mut commands, &game_state);

                    }

                    // Démarage du combat 
                    GameButtonAction::StartArenaCombat => {
                        game_state.show_intro_screen = false;

                         for entity in arena_ui_query.iter() {
                            commands.entity(entity).despawn_recursive();
                        }

                        spawn_arena_combat_screen(&mut commands, &game_state);
                    }
                    // Sélection d'ingrédients pour le cocktail
                    GameButtonAction::SelectIngredient(ingredient) => {
                        // ajoute l'ingredient selectionne 
                        if !game_state.current_crafting.selected_ingredients.contains(ingredient) {
                            game_state.current_crafting.selected_ingredients.push(ingredient.clone());
                        }

                        // netoyage de l'ecran actuel
                        for entity in gameentities.iter() {
                            commands.entity(entity).despawn_recursive();
                        }

                        // rafraichissement de l'ecran de combat
                        spawn_arena_combat_screen(&mut commands, &game_state);
                    }
                    // Validation du cocktail créé
                    GameButtonAction::ValidateCocktail => {
               
                    if let Some(recipe) = &game_state.master_recipe {
                        let selected: HashSet<String> = game_state.current_crafting.selected_ingredients.iter().cloned().collect();
                        let expected: HashSet<String> = recipe.ingredients.iter().map(|i| i.name.clone()).collect();

                        if selected == expected {
                            // Cocktail correct
                            game_state.current_crafting.correct = true;
                            game_state.boss_hp /= 2;
                            game_state.show_crafting_phase = true;

                            game_state.current_crafting.selected_ingredients.clear();

                              // nettoyage l'ecran actuel
                            for entity in gameentities.iter() {
                                commands.entity(entity).despawn_recursive();
                            }

                            // on rafraichis l'ecran de combat
                            spawn_arena_combat_screen(&mut commands, &game_state);
                        }
                    }
                    else {
                         // Cocktail incorrect
                        game_state.current_crafting.correct = false;
                        info!("Mauvais cocktail ! Le boss contre-attaque !");
                        game_state.player_hp -= 20;

                        if game_state.player_hp <= 0 {
                            game_state.arena_combat_state = ArenaCombatState::Defeat;
                            info!("Defaite ! Vous avez ete battu.");
                        }
                    }

                    // nettoyage l'ecran actuel
                    for entity in gameentities.iter() {
                        commands.entity(entity).despawn_recursive();
                    }

                    // Rafraîchissement de l'écran
                    spawn_arena_combat_screen(&mut commands, &game_state);
                }
                    GameButtonAction::BackToMainFromCombat => {
                    // Nettoyer l'ecran actuel
                    for entity in gameentities.iter() {
                        commands.entity(entity).despawn_recursive();
                    }

                    // Retour au menu principal depuis le combat
                    game_state.current_screen = GameScreenType::Main;
                    spawn_main_game_screen(&mut commands);
                }
                // Démarrage de la phase finale de craft
                GameButtonAction::StartFinalCraft=> {
                     game_state.show_crafting_phase = false;
                    for entity in arena_ui_query.iter() {
                            commands.entity(entity).despawn_recursive();
                    }
                    spawn_arena_crafting_phase_screen(&mut commands, &game_state);
                }
                // Sélection d'instructions
                GameButtonAction::SelectInstruction(instruction) => {
                    if !game_state.current_crafting.selected_instructions.contains(&instruction) {
                        game_state.current_crafting.selected_instructions.push(instruction.to_string());
                    }
                },

                // Validation de l'ordre des instructions

                GameButtonAction::ValidateInstructionOrder => {
                    if let Some(recipe) = &game_state.master_recipe {
                        let expected = &recipe.instructions;
                        let selected = &game_state.current_crafting.selected_instructions;

                        if selected == expected {
                            println!("✅ Ordre des instructions correct !");
                            game_state.current_crafting.instruction_correct = true;
                            game_state.boss_hp = 0;
                            for entity in arena_ui_query.iter() {
                                    commands.entity(entity).despawn_recursive();
                            }
                            spawn_arenaend_screen(&mut commands, &game_state);
                        } else {
                            println!("❌ Ordre des instructions incorrect !");
                            game_state.current_crafting.instruction_correct = false;
                        }
                    }
                },

                     // Gestion des réponses aux questions
                    GameButtonAction::AnswerQuestion(answer_index) => {
                                        let selected_answer = &game_state.answer_options[*answer_index];
                        
                                        for entity in gameentities.iter() {
                                            commands.entity(entity).despawn_recursive();
                                        }
                        
                                     if *selected_answer == game_state.correct_answer {
                                        // Bonne réponse on charge les données du maître

                                        game_state.current_screen = GameScreenType::ArenaPresentation;

                                        info!("Bonne reponse : on passe à l'ecran de presentation d'arene.");

                                        match JsonLoader::loadJsonMasters("assets/caracters/pnj/masters.json") {
                                            Ok(masters) => {
                                                info!("Masters charges avec succes : {} masters trouves.", masters.len());

                                                if let Some(selected_index) = game_state.selected_arena_index {
                                                    info!("Index de l'arene selectionnee : {}", selected_index);

                                                    if let Some(master) = masters.get(selected_index) {
                                                        info!("Master trouve pour l'arene {} : {}", selected_index, master.pnj.caracter.name);
                                                         // Chargement des données du maître

                                                        game_state.master_name = Some(master.pnj.caracter.name.clone());
                                                        game_state.master_style = Some(master.pnj.caracter.style.clone());
                                                        game_state.master_badge = Some(master.badge.name.clone());
                                                        game_state.master_attacks = master.attacks.clone();
                                                        game_state.master_dialogs = master.pnj.dialogs.clone();
                                                        game_state.master_recipe = Some(master.recipe.clone());
                                                    } else {
                                                        info!("Aucun master trouve pour l'index {} ! Utilisation d'un master fictif.", selected_index);

                                                        // Données de fallback

                                                        game_state.master_name = Some("Master Inconnu".to_string());
                                                        game_state.master_style = Some("Style Mystere".to_string());
                                                        game_state.master_badge = Some("Badge Inconnu".to_string());
                                                        game_state.master_attacks = vec!["?".to_string()];
                                                        game_state.master_dialogs = vec!["...".to_string()];
                                                    }
                                                } else {
                                                    info!("Aucun index d'arene selectionne ! Utilisation d'un master fictif par defaut.");

                                                    // Données de fallback
                                                    game_state.master_name = Some("Master Inconnu".to_string());
                                                    game_state.master_style = Some("Style Mystere".to_string());
                                                    game_state.master_badge = Some("Badge Inconnu".to_string());
                                                    game_state.master_attacks = vec!["?".to_string()];
                                                    game_state.master_dialogs = vec!["...".to_string()];
                                                }
                                            }
                                            Err(e) => {
                                                // Données d'erreur

                                                info!("Erreur lors du chargement des masters : {:?}.", e);

                                                game_state.master_name = Some("Master Erreur".to_string());
                                                game_state.master_style = Some("Erreur de Chargement".to_string());
                                                game_state.master_badge = Some("Badge Manquant".to_string());
                                                game_state.master_attacks = vec!["Erreur".to_string()];
                                                game_state.master_dialogs = vec!["Impossible de charger le master".to_string()];
                                            }
                                        }

                                        spawn_arena_presentation_screen(&mut commands, &game_state);
                                    }

                                    else {
                                            // Mauvaise réponse on reviens à la sélection d'arène
                                            game_state.wrong_answer_message = true;
                                            game_state.current_screen = GameScreenType::ArenaSelection;
                                            spawn_arena_selection_screen(&mut commands, &game_state);
                                        }
                                    }
                    // Navigation de retour
                    GameButtonAction::BackToArenaSelection => {
                                        game_state.wrong_answer_message = false;
                        
                                        for entity in gameentities.iter() {
                                            commands.entity(entity).despawn_recursive();
                                        }
                        
                                        game_state.current_screen = GameScreenType::ArenaSelection;
                                        spawn_arena_selection_screen(&mut commands, &game_state);
                                    }
                    GameButtonAction::BackToMainGame => {
                                        game_state.wrong_answer_message = false;
                        
                                        for entity in gameentities.iter() {
                                            commands.entity(entity).despawn_recursive();
                                        }
                        
                                        game_state.current_screen = GameScreenType::Main;
                                        spawn_main_game_screen(&mut commands);
                                    }
                                            GameButtonAction::StartCombat => todo!(),
                }
                
                *background_color = Color::srgb(0.3, 0.3, 0.5).into();
            }
            // États visuels des boutons
            Interaction::Hovered => {
                *background_color = Color::srgb(0.25, 0.25, 0.35).into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
            }
        }
    }
}
// On nettoie l'écran de jeu en supprimant toutes les entités associées
pub fn despawn_game(mut commands: Commands, query: Query<Entity, With<GameScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Marqueur de composant pour l'écran de présentation d'arène
#[derive(Component)]
pub struct ArenaPresentationUI;
// On crée l'écran de présentation de l'arène avec les informations du maître
fn spawn_arena_presentation_screen(commands: &mut Commands, game_state: &GameScreenState) {
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
        BackgroundColor(Color::srgb(0.15, 0.15, 0.25)),
        GameScreen,
        ArenaPresentationUI,
    ))
    .with_children(|parent| {
        // Titre de l'arène
        if let Some(selected_arena) = &game_state.selected_arena {
            parent.spawn(Text::new(format!("Presentation de l'Arene: {}", selected_arena)));
        } else {
            parent.spawn(Text::new("Presentation de l'Arene"));
        }

        // Informations du maître

        if let Some(master_name) = &game_state.master_name {
            parent.spawn(Text::new(format!("Maître de l'Arene: {}", master_name)));
            parent.spawn(Text::new(game_state.master_dialogs.join("\n")));
    
        }
        if let Some(master_style) = &game_state.master_style {
            parent.spawn(Text::new(format!("Style: {}", master_style)));
        }

        if let Some(master_badge) = &game_state.master_badge {
            parent.spawn(Text::new(format!("Badge: {}", master_badge)));
        }

        // Liste des attaques


        parent.spawn(Text::new("Attaques:"));
        for attack in &game_state.master_attacks {
            parent.spawn(Text::new(format!("- {}", attack)));
        }

        // Bouton pour continuer vers l'Arene
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
                GameButtonAction::EncounterBouncer,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child(Text::new("Continuer"));
    });
}

// États possibles du combat d'arène
#[derive(Default, PartialEq)]
pub enum ArenaCombatState {
    #[default]
    Start,
    PlayerTurn,
    BossTurn,
    Victory,
    Defeat,
}

// Structure qui représente l'état de création d'un cocktail
#[derive(Default)]
pub struct CurrentCocktailCrafting {
    pub selected_ingredients: Vec<String>,
    pub completed: bool,
    pub correct: bool,
    pub selected_instructions: Vec<String>,
    pub instruction_correct: bool,
}

// Crée l'écran de combat d'arène avec les différentes phases de jeu
fn spawn_arena_combat_screen(commands: &mut Commands, game_state: &GameScreenState) {
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
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        GameScreen,
        ArenaUI,
    ))
    .with_children(|parent| {
        // Écran d'introduction au combat

        if game_state.show_intro_screen { 
            // Titre
            parent.spawn(Text::new(format!(
                "Vous allez affronter {} sur {}",
                game_state.master_name.as_deref().unwrap_or("???"),
                game_state.selected_arena.as_deref().unwrap_or("???"),
            )));

            // Bouton de démarrage du combat
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
                    GameButtonAction::StartArenaCombat,
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child(Text::new("Commencer le combat"));

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
                    GameButtonAction::BackToMainFromCombat,
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child(Text::new("Retour"));

            return;
        // Phase de transition après validation du cocktail
        } else if game_state.show_crafting_phase {
            parent.spawn(Text::new(
                "🎉 Bien joue ! Tu as trouve la bonne recette.\nMaintenant concocte le cocktail comme il faut pour finir le boss."
            ));

            parent
                .spawn((Button, GameButtonAction::StartFinalCraft))
                .with_child(Text::new("Continuer"));

            return;
        }

        // On affiche les points de vie

        if let Some(master_name) = &game_state.master_name {
            parent.spawn(Text::new(format!("Combat contre le Maître: {}", master_name)));
        } else {
            parent.spawn(Text::new("Combat d'Arene"));
        }

        // Affichage des points de vie
        parent.spawn(Text::new(format!("Votre HP: {}", game_state.player_hp)));
        parent.spawn(Text::new(format!("HP du Boss: {}", game_state.boss_hp)));

        parent.spawn(Text::new("Selectionnez les ingredients pour le cocktail:"));

        // Liste des ingrédients disponibles
        let static_ingredients = vec![
            "Jus de citron",
            "Vodka",
            "Rhum",
            "Menthe",
            "Sirop de sucre",
            "Eau gazeuse",
            "Tequila",
            "Triple sec",
        ];

        let mut all_ingredients_set: HashSet<String> = static_ingredients
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        // Ajout des ingrédients de la recette du maître
        if let Some(recipe) = &game_state.master_recipe {
            for ingredient in &recipe.ingredients {
                all_ingredients_set.insert(ingredient.name.clone());
            }
        }

        let mut all_ingredients: Vec<String> = all_ingredients_set.into_iter().collect();
        all_ingredients.sort();

        println!("Liste finale des ingredients affiches : {:?}", all_ingredients);

        // On crée les boutons d'ingrédients
        for ingredient in all_ingredients {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(40.0),
                        margin: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                    GameButtonAction::SelectIngredient(ingredient.to_string()),
                ))
                .with_child(Text::new(ingredient));
        }

        // Affichage des ingrédients sélectionnés
        parent.spawn(Text::new(format!(
            "Ingredients selectionnes: {:?}",
            game_state.current_crafting.selected_ingredients
        )));

        // Bouton de validation du cocktail
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
                GameButtonAction::ValidateCocktail,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child(Text::new("Valider le Cocktail"));

        // Validation du cocktail
        if let Some(recipe) = &game_state.master_recipe {
            let selected = &game_state.current_crafting.selected_ingredients;
            let expected: HashSet<String> = recipe.ingredients.iter().map(|i| i.name.clone()).collect();

            let correct_count = selected.iter().filter(|i| expected.contains(*i)).count();
            let incorrect_count = selected.len() - correct_count;

            let is_valid = selected.len() == expected.len() && incorrect_count == 0;

            println!(
                "Validation cocktail — bons: {}, mauvais: {}, selection: {:?}, attendu: {:?}",
                correct_count, incorrect_count, selected, expected
            );

            let validation_text = if is_valid {
                "✅ Cocktail valide !".to_string()
            } else {
                format!("❌ Cocktail incorrect : {} bon(s), {} mauvais.", correct_count, incorrect_count)
            };

            parent.spawn(Text::new(validation_text));
        } else {
            parent.spawn(Text::new("Aucune recette de maître disponible."));
        }

        // Bouton de retour
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
                GameButtonAction::BackToMainFromCombat,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child(Text::new("Retour"));
    });
}

// On crée l'écran de phase finale de craft avec l'ordre des instructions
fn spawn_arena_crafting_phase_screen(commands: &mut Commands, game_state: &GameScreenState) {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

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
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        GameScreen,
        ArenaUI,
    ))
    .with_children(|parent| {
        parent.spawn(Text::new("Derniere etape : remettre les instructions dans le bon ordre !"));

        if let Some(recipe) = &game_state.master_recipe {

            // Mélange aléatoire des instructions pour le défi
            let mut shuffled_instructions = recipe.instructions.clone();
            shuffled_instructions.shuffle(&mut thread_rng());

            parent.spawn(Text::new("Cliquez sur les etapes dans l'ordre :"));

            // Création des boutons d'instructions
            for (index, instruction) in shuffled_instructions.iter().enumerate() {
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(400.0),
                            height: Val::Px(40.0),
                            margin: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        BorderColor(Color::BLACK),
                        BorderRadius::MAX,
                        BackgroundColor(NORMAL_BUTTON),
                        GameButtonAction::SelectInstruction(instruction.clone()),
                    ))
                    .with_child(Text::new(format!("Etape {} : {}", index + 1, instruction)));
            }

            // Affichage de l'ordre sélectionné
            parent.spawn(Text::new(format!(
                "Ordre selectionne: {:?}",
                game_state.current_crafting.selected_instructions
            )));

            // Bouton de validation de l'ordre
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
                    GameButtonAction::ValidateInstructionOrder,
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child(Text::new("Valider l'ordre"));

            // Affichage du résultat si valide
            if game_state.current_crafting.instruction_correct {
                parent.spawn(Text::new("✅ Bravo, vous avez fini le boss !"));
            }
        } else {
            parent.spawn(Text::new("Aucune recette disponible."));
        }

        // Bouton de retour
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
                GameButtonAction::BackToMainFromCombat,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child(Text::new("Retour"));
    });
}

/// On crée l'écran de fin d'arène après la victoire

fn spawn_arenaend_screen(commands: &mut Commands, game_state: &GameScreenState) {
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
        BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
        GameScreen,
        ArenaUI,
    ))
    .with_children(|parent| {
        // Récapitulatif de la victoire
        parent.spawn(Text::new("🎉 Bravo ! Vous avez battu le boss ! 🏆"));

        parent.spawn(Text::new(format!(
            "Maître battu : {}\nArene : {}",
            game_state.master_name.as_deref().unwrap_or("???"),
            game_state.selected_arena.as_deref().unwrap_or("???"),
        )));

        // Bouton de retour à la sélection des niveaux

        parent
            .spawn((
                Button,
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(60.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                GameButtonAction::SelectArena, 
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child(Text::new("Retour à la selection des niveaux"));
    });
}
