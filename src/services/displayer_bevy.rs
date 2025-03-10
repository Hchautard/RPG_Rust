use bevy::prelude::*;
use bevy::ecs::prelude::*;
use std::io;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect};
use crate::models::aptitude::Aptitude;

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
}



fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}



fn button_system(
    mut interaction_query: Query<(
        &Interaction, 
        &ButtonAction, 
        &mut BackgroundColor,
        &mut BorderColor,
    ), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
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
                    ButtonAction::NewGame | ButtonAction::LoadGame => {
                        app_state.set(AppState::Game);
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
        
        // Spawn buttons
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
        // Display aptitude list
        for aptitude in &aptitude_list.aptitudes {
            parent.spawn(Text::from(
                aptitude.name.clone(),
            ));
        }

        // Back button
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
