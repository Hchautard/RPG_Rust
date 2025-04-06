use bevy::prelude::*;
use bevy::ecs::prelude::*;
use std::io;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect};
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
    StartFight
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
    Fight
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const RED: Color = Color::srgb(1.0, 0.0, 0.0);
const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
const BLUE: Color = Color::srgb(0.0, 0.0, 1.0);
const WHITE: Color = Color::srgb(1.0, 1.0, 1.0);

pub struct DisplayerBevy;

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
            .add_systems(OnExit(AppState::Aptitudes), despawn_screen::<AptitudesScreen>);
    }
}

impl DisplayerBevy {
    pub fn new() -> Self {
        DisplayerBevy
    }

    pub fn run(&self, aptitudes: &[Aptitude]) -> io::Result<()> {

        App::new()
            .add_plugins(DefaultPlugins)
            .insert_resource(AptitudeList { aptitudes: aptitudes.to_vec() })
            .add_plugins(DisplayerBevy::new())
            .run();

        Ok(())
    }

    pub fn show_game(&self) {

    }
}



fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}



fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<(
        &Interaction, 
        &ButtonAction, 
        &mut BackgroundColor,
        &mut BorderColor,
    ), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut combat_state: Option<ResMut<CombatState>>, 
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
                                        let badge = Badge::new("Badge de Débutant", vec!["Tu es prêt à commencer !".to_string()]);
                                        let master = Master::new("Gin Tonic", "Mixologue",50, 50, 20, "Boss", vec!["Tu ne me battras jamais !".to_string()], badge.clone(), vec!["Coup de Gin".to_string()]);
                                        let player = Player::new("Héros", "Barman", badge, vec![], vec![], 60);
                                        start_combat(&mut commands, player, master, &mut app_state);
                                    }
                    ButtonAction::ShowAptitudes => {
                                        app_state.set(AppState::Aptitudes);
                                    }
                    ButtonAction::Quit => {
                                        std::process::exit(0);
                                    }
                    ButtonAction::Back => {
                                        app_state.set(AppState::MainMenu);
                                    }
                    ButtonAction::StartFight => {
                                        if let Some(combat) = combat_state.as_deref_mut() {
                                            combat.started = true;
                                        }
                                    }                               
                    ButtonAction::LoadGame => todo!(),
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

pub fn start_combat(commands: &mut Commands, mut player: Player, mut master: Master, app_state: &mut ResMut<NextState<AppState>>) {
    player.caracter.hp = player.max_hp;
    master.pnj.caracter.hp = master.max_hp;

    commands.insert_resource(CombatState {
        player,
        master,
        turn: Turn::Player,
        finished: false,
        started: false,
    });

    app_state.set(AppState::Fight);
}




fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        
        for (label, action) in [
            ("Nouvelle Partie", ButtonAction::NewGame),
            ("Charger Partie", ButtonAction::LoadGame),
            ("Voir les Aptitudes", ButtonAction::ShowAptitudes),
            ("Quitter", ButtonAction::Quit),
        ] {
            parent
                .spawn((
                    Button {
                        ..Default::default()
                    },
                    action,
                ))
                .with_children(|button| {
                    button.spawn(Text::from(label));
                }); ;
        }
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
            .with_child((
                Text::new("Retour"),
            ));
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
) {
    if !combat.started {
        return;
    }

    
    if combat.finished {
        return;
    }

    if combat.player.caracter.hp <= 0 {
        println!("Le joueur a perdu !");
        combat.finished = true;
        //next_state.set(AppState::MainMenu);
        return;
    }

    if combat.master.pnj.caracter.hp <= 0 {
        println!("Le joueur a battu {} et gagne le badge {:?}", combat.master.pnj.caracter.name, combat.master.badge);
        combat.finished = true;
        //next_state.set(AppState::MainMenu);
        return;
    }

    match combat.turn {
        Turn::Player => {
            println!("C'est au joueur de préparer un cocktail !");
            let damage = 10;
            combat.master.pnj.caracter.hp -= damage;
            println!("Le joueur inflige {} de dégâts à {}", damage, combat.master.pnj.caracter.name);
            combat.turn = Turn::Master;
        }
        Turn::Master => {
            println!("{} prépare un cocktail dévastateur!", combat.master.pnj.caracter.name);
            let damage = 8; 
            combat.player.caracter.hp -= damage;
            println!("{} inflige {} de dégâts au joueur", combat.master.pnj.caracter.name, damage);
            combat.turn = Turn::Player;
        }
    }
}

#[derive(Component)]
struct CombatScreen;

fn setup_combat_screen(mut commands: Commands, combat: Res<CombatState>) {
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
        parent.spawn(Text::from(format!("HP Joueur : {}", combat.player.caracter.hp)));
        parent.spawn(Text::from(format!("HP Boss : {}", combat.master.pnj.caracter.hp)));

        parent.spawn((
            Button,
            ButtonAction::Back,
            BackgroundColor(NORMAL_BUTTON),
        )).with_children(|b| {
            b.spawn(Text::from("Fuir"));
        });
        parent.spawn((
            Button,
            ButtonAction::StartFight,
            BackgroundColor(NORMAL_BUTTON),
        )).with_children(|b| {
            b.spawn(Text::from("Commencer le Combat"));
        });
        
    });
}

