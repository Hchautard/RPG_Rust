use bevy::prelude::*;
use bevy::ecs::prelude::*;
use std::io;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect};
use bevy::image::Image;
use crate::models::caracter::caracter::Caracter;
use crate::models::caracter::pnj::Pnj;
use crate::models::{aptitude::Aptitude, badge::Badge, caracter::{master::Master, player::Player}};
use super::combat_state::{CombatState, Turn};

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct AptitudesScreen;

#[derive(Component)]
enum ButtonAction {
    NewGame,
    LoadGame,
    ShowAptitudes,
    Quit,
    Back,
    StartFight,
    SelectLevel(Level),  
    Locked,             
}

#[derive(Resource)]
pub struct AptitudeList {
    aptitudes: Vec<Aptitude>,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    Aptitudes,
    Game,
    Fight,
    Levels, 
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const RED: Color = Color::srgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
const BLUE: Color = Color::srgb(0.0, 0.0, 1.0);
const WHITE: Color = Color::srgb(1.0, 1.0, 1.0);

pub struct DisplayerBevy {
   pub masters: Vec<Master>,
}
impl Plugin for DisplayerBevy {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(Startup, setup)
            .add_systems(Update, button_system)
            .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_screen::<MainMenu>)
            .add_systems(OnEnter(AppState::Aptitudes), setup_aptitudes_screen)
            .add_systems(OnEnter(AppState::Fight), setup_combat_screen)
            .add_systems(OnExit(AppState::Fight), despawn_screen::<CombatScreen>)
            .add_systems(Update, combat_system.run_if(in_state(AppState::Fight)))
            .add_systems(OnExit(AppState::Aptitudes), despawn_screen::<AptitudesScreen>)
            .add_systems(OnEnter(AppState::Levels), setup_levels_screen); 
    }
}

impl DisplayerBevy {
    pub fn new(masters: Vec<Master>) -> Self {
            DisplayerBevy {
                masters: masters,
            }
        }

    pub fn run(&self, aptitudes: &[Aptitude]) -> io::Result<()> {

        App::new()
            .add_plugins(DefaultPlugins)
            .insert_resource(AptitudeList { aptitudes: aptitudes.to_vec() })
            .insert_resource(MasterList { masters: self.masters.clone() })  
            .add_plugins(DisplayerBevy::new(Vec::new()))
            .insert_resource(LevelList { levels: Vec::new() })
            .run();

        Ok(())
    }


}



fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}


fn button_system(
    mut commands: Commands,
    masters: Res<MasterList>,
    mut interaction_query: Query<(
        &Interaction, 
        &ButtonAction, 
        &mut BackgroundColor,
        &mut BorderColor,
    ), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut combat_state: Option<ResMut<CombatState>>, 
    mut level_list: ResMut<LevelList>,  // Accès aux niveaux
) {
    for (
        interaction, 
        action, 
        mut background_color, 
        mut border_color,
    ) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::srgb(0.5, 0.5, 0.8).into();
                border_color.0 = RED.into();
                match action {
                    ButtonAction::NewGame => {
                    
                        app_state.set(AppState::Levels);
                    },
                    ButtonAction::SelectLevel(level) => {
                          start_combat(&mut commands, &masters.masters[level.index], &mut app_state);
                       
                        
                   
                    }
                    ButtonAction::Locked => {
                        println!("Ce niveau est verrouillé!");
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                *background_color = Color::srgb(0.5, 0.5, 0.8).into();
                border_color.0 = GREEN.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
                border_color.0 = WHITE.into();
            }
        }
    }
}


pub fn start_combat(commands: &mut Commands, mut master: &Master, app_state: &mut ResMut<NextState<AppState>>) {
   let player = Player {
        caracter: Caracter {
            name: "Joueur".to_string(),
            style: "Joueur".to_string(),
            hp: 100,
            pp: 100,
            bankroll: 0,
        },
        max_hp: 100,
        badge: Badge::new("name", Vec::new()),
        inventory: vec![],
        level: 1,
        reputation: 0,
        aptitudes: vec![],
   };
    

    master.clone().pnj.caracter.hp = master.max_hp;

    commands.insert_resource(CombatState {
        player,
        turn: Turn::Player,
        finished: false,
        started: false,
        master: master.clone(),
    });

    app_state.set(AppState::Fight);
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>, mut app_state: ResMut<NextState<AppState>>) {
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
        parent.spawn(Button {
            ..Default::default()
        })
        .with_children(|button| {
            button.spawn(Text::from("Nouvelle Partie"));
        })
        .insert(ButtonAction::NewGame);
    });
}


fn setup_aptitudes_screen(mut commands: Commands, aptitude_list: Res<AptitudeList>) {
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
        BackgroundColor(Color::rgb(0.2, 0.2, 0.2)),
        AptitudesScreen,

    ))
    .with_children(|parent| {
        for aptitude in &aptitude_list.aptitudes {
            parent.spawn(Text::from(
                aptitude.name.clone(),
            ));
        }

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
            .with_children(|b| {
                b.spawn(Text::from("Retour"));
            });
            
    });
}


fn despawn_screen<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
fn combat_system(
    mut combat: ResMut<CombatState>,
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands, 
    combat_query: Query<Entity, With<CombatScreen>>,
) {
    if !combat.started {
        return;
    }

    if combat.finished {
        return;
    }

    // Update the combat screen with the current HP
    for combat_screen in combat_query.iter() {
        commands.entity(combat_screen).despawn_recursive();
    }

    let combat_screen = commands.spawn((Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        ..Default::default()
    }, CombatScreen)).id();

    commands.entity(combat_screen).with_children(|parent| {
        parent.spawn(Text::from(format!("Combat contre : {}", combat.master.pnj.caracter.name)));
        parent.spawn(Text::from(format!("HP Joueur: {}", combat.player.caracter.hp)));
        parent.spawn(Text::from(format!("HP Boss: {}", combat.master.pnj.caracter.hp)));
        
        // Add action buttons
        parent.spawn((Button, ButtonAction::StartFight, BackgroundColor(NORMAL_BUTTON)))
            .with_children(|button| {
                button.spawn(Text::from("Attaquer"));
            });
        
        parent.spawn((Button, ButtonAction::Back, BackgroundColor(NORMAL_BUTTON)))
            .with_children(|button| {
                button.spawn(Text::from("Retour"));
            });
    });

    if combat.player.caracter.hp <= 0 {
        println!("Le joueur a perdu !");
        combat.finished = true;
        next_state.set(AppState::MainMenu); // Go back to the main menu when finished
        return;
    }

    if combat.master.pnj.caracter.hp <= 0 {
        println!("Le joueur a gagné !");
        combat.finished = true;
        next_state.set(AppState::MainMenu); // Go back to the main menu when finished
        return;
    }

    match combat.turn {
        Turn::Player => {
            let damage = 10;
            combat.master.pnj.caracter.hp -= damage;
            println!("Le joueur inflige {} de dégâts à {}", damage, combat.master.pnj.caracter.name);
            combat.turn = Turn::Master;
        }
        Turn::Master => {
            let damage = 8;
            combat.player.caracter.hp -= damage;
            println!("{} inflige {} de dégâts au joueur", combat.master.pnj.caracter.name, damage);
            combat.turn = Turn::Player;
        }
    }
}


#[derive(Component)]
struct CombatScreen;fn setup_combat_screen(
    mut commands: Commands, 
    combat: Res<CombatState>,
    asset_server: Res<AssetServer>,
) {
    let master_image: Handle<Image> = asset_server.load("images/OIP.png");
    let cocktail_image: Handle<Image> = asset_server.load("images/OIP.png");

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

        parent.spawn(ImageNode {
            image: master_image.clone(),
            ..Default::default()
        });

        parent.spawn(Text::from(format!("HP Joueur : {}", combat.player.caracter.hp)));
        parent.spawn(Text::from(format!("HP Boss : {}", combat.master.pnj.caracter.hp)));

        parent.spawn((
            Button,
            ButtonAction::Back,
            BackgroundColor(NORMAL_BUTTON),
        ))
        .with_children(|b| {
            b.spawn(Text::from("Fuir"));
        });

        parent.spawn((
            Button,
            ButtonAction::StartFight,
            BackgroundColor(NORMAL_BUTTON),
        ))
        .with_children(|b| {
            b.spawn(Text::from("Commencer le Combat"));
        });
    });
}


#[derive(Clone, PartialEq)]
pub struct Level {
    pub index: usize,
    pub name: String,
    pub image: Handle<Image>,
    pub is_locked: bool, 
    pub boss_name: String,
}


#[derive(Resource)]
pub struct LevelList {
    pub levels: Vec<Level>,
}

#[derive(Resource)]
pub struct MasterList {
    pub masters: Vec<Master>,
}

fn load_levels(asset_server: &Res<AssetServer>) -> Vec<Level> {
    vec![
        Level {
            index: 0,
            name: "Niveau 1".to_string(),
            image: asset_server.load("images/OIP.png"),
            is_locked: false,  // Le premier niveau est déverrouillé
            boss_name: "Boss 1".to_string(),
        },
        Level {
            index: 1,
            name: "Niveau 2".to_string(),
            image: asset_server.load("images/boss2.png"),
            is_locked: true,  // Le deuxième niveau est verrouillé
            boss_name: "Boss 2".to_string(),
        },
        // Ajoutez les autres niveaux ici
    ]
}

fn setup_levels_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut level_list: ResMut<LevelList>,
) {
    // Charger les niveaux
    level_list.levels = load_levels(&asset_server);

    // Créer l'UI
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
    ))
    .with_children(|parent| {
        // Afficher chaque niveau sous forme de bouton avec une image
        for level in &level_list.levels {
            let button_interaction = if level.is_locked {
                ButtonAction::Locked
            } else {
                ButtonAction::SelectLevel(level.clone())
            };

            parent
                .spawn((
                    Button {
                        ..Default::default()
                    },
                    button_interaction,
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


#[derive(Resource)]
pub struct PlayerResource {
    player: Player,
}