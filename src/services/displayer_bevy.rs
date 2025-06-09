use bevy::prelude::*;
use std::io;
use crate::models::aptitude::Aptitude;

// Import des constantes et états
use crate::services::ui::constants::{
    AppState, ButtonAction, GameLoadContext,
    NORMAL_BUTTON, WHITE, RED, GREEN
};
// Import des ressources et composants depuis les sous-modules
use crate::services::ui::aptitudes_screen::{AptitudeList, setup_aptitudes_screen, despawn_aptitudes_screen};
use crate::services::ui::main_menu::{setup_main_menu, despawn_main_menu};
use crate::services::ui::game_screen::{
    setup_game, despawn_game, handle_game_button_actions, GameScreenState
};
use crate::services::ui::player_slot_screen::{PlayerSlotScreenPlugin, SelectedPlayerSlot};
use crate::services::ui::player_creation_screen::{PlayerCreationPlugin, PlayerCreationData, create_player};
use crate::services::ui::start_screen::StartScreenPlugin;

pub struct DisplayerBevy;

impl Plugin for DisplayerBevy {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_resource::<GameLoadContext>()
            .init_resource::<GameScreenState>()  
            .add_systems(Startup, setup)
            .add_systems(Update, (button_system, handle_game_button_actions))

            // Menu principal
            .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)

            // Écran des aptitudes
            .add_systems(OnEnter(AppState::Aptitudes), setup_aptitudes_screen)
            .add_systems(OnExit(AppState::Aptitudes), despawn_aptitudes_screen)

            // Écran de jeu
            .add_systems(OnEnter(AppState::Game), setup_game)
            .add_systems(OnExit(AppState::Game), despawn_game)
            
            // Ajout du plugin pour les slots de joueur
            .add_plugins(PlayerSlotScreenPlugin)
            
            // Ajout du plugin pour la création de personnage
            .add_plugins(PlayerCreationPlugin)
            
            // Ajout du plugin pour l'écran de démarrage (contenu de sauvegarde)
            .add_plugins(StartScreenPlugin);
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
            .init_resource::<SelectedPlayerSlot>()
            .init_resource::<PlayerCreationData>()
            .init_resource::<GameLoadContext>()
            .init_resource::<GameScreenState>()
            .add_plugins(DisplayerBevy::new())
            .run();

        Ok(())
    }
}

#[derive(Resource, Clone)]
pub struct UIFont(pub Handle<Font>);


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    let font_handle = asset_server.load("fonts/NotoSans-Regular.ttf");

    commands.insert_resource(UIFont(font_handle));
}



fn button_system(
    mut interaction_query: Query<(
        &Interaction, 
        &ButtonAction, 
        &mut BackgroundColor,
        &mut BorderColor,
    ), (Changed<Interaction>, With<Button>)>,
    mut app_state: ResMut<NextState<AppState>>,
    mut selected_slot: ResMut<SelectedPlayerSlot>,
    mut game_load_context: ResMut<GameLoadContext>,
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
                border_color.0 = RED;
                
                // Gestion des actions
                match action {
                    ButtonAction::NewGame => {
                        game_load_context.is_load_game = false;
                        app_state.set(AppState::PlayerSlot);
                    }
                    ButtonAction::LoadGame => {
                        game_load_context.is_load_game = true;
                        app_state.set(AppState::PlayerSlot);
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
                    ButtonAction::SelectSlot(slot_index) => {
                        selected_slot.slot = Some(*slot_index);
                    }
                    ButtonAction::ConfirmSlot => {
                        if selected_slot.slot.is_some() {
                            if game_load_context.is_load_game {
                                app_state.set(AppState::StartScreen);
                            } else {
                                app_state.set(AppState::PlayerCreation);
                            }
                        }
                    }
                    ButtonAction::CreatePlayer => {
                        let _player = create_player(&PlayerCreationData::default());
                        app_state.set(AppState::Game);
                    }
                    ButtonAction::StartGame => {
                        app_state.set(AppState::Game);
                    }
                }
            }
            Interaction::Hovered => {
                *background_color = Color::srgb(0.5, 0.5, 0.8).into();
                border_color.0 = GREEN;
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
                border_color.0 = WHITE;
            }
        }
    }
}