use bevy::prelude::*;
use std::io;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect};
use bevy::image::Image;
use crate::models::caracter::caracter::Caracter;
use crate::models::caracter::player;
use crate::services::ui::constants::{
    AppState, ButtonAction, Level, BLACK, BLUE, GREEN, NORMAL_BUTTON, RED, SELECTED_BUTTON, WHITE
};
use crate::models::{aptitude::Aptitude, badge::Badge, caracter::{master::Master, player::Player}};
use super::combat_state::{CombatState, Turn};
use super::ui::constants::{LevelList, MasterList};
use crate::services::ui::aptitudes_screen::{AptitudeList, setup_aptitudes_screen, despawn_aptitudes_screen};
use crate::services::ui::player_slot_screen::{PlayerSlotScreenPlugin, SelectedPlayerSlot};
use crate::services::ui::select_level::{ LevelsScreenPlugin};
use crate::services::ui::combat_screen::{ CombatScreenPlugin};


use crate::services::ui::main_menu::{setup_main_menu, despawn_main_menu};
use crate::services::ui::game_screen::{setup_game, despawn_game};
use crate::services::ui::player_creation_screen::{PlayerCreationPlugin, PlayerCreationData, create_player};

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct AptitudesScreen;

pub struct DisplayerBevy {
   pub masters: Vec<Master>,
}
impl Plugin for DisplayerBevy {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(Startup, setup)
            .add_systems(Update, button_system)
            .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)

            .add_systems(OnEnter(AppState::Aptitudes), setup_aptitudes_screen)
            .add_systems(OnExit(AppState::Aptitudes), despawn_aptitudes_screen)

            .add_systems(OnEnter(AppState::Game), setup_game)
            .add_systems(OnExit(AppState::Game), despawn_game)
            .add_plugins(LevelsScreenPlugin)
            .add_plugins(CombatScreenPlugin)
            // Ajout du plugin pour les slots de joueur
            .add_plugins(PlayerSlotScreenPlugin)
            
            // Ajout du plugin pour la création de personnage
            .add_plugins(PlayerCreationPlugin);
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
            .init_resource::<SelectedPlayerSlot>()
            .init_resource::<PlayerCreationData>()
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
    mut level_list: ResMut<LevelList>,  // Accès aux niveaux
    mut selected_slot: ResMut<SelectedPlayerSlot>,
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
                        // Aller à l'écran de sélection de slot au lieu d'aller directement au jeu
                        app_state.set(AppState::PlayerSlot);
                    }
                    ButtonAction::ShowAptitudes => {
                        app_state.set(AppState::Aptitudes);
                    }
                    ButtonAction::LoadGame => {
                        // Pour le chargement aussi, on passe par la sélection de slot
                        app_state.set(AppState::PlayerSlot);
                    }
                    ButtonAction::Locked => {
                        println!("Ce niveau est verrouillé!");
                    }
                    ButtonAction::SelectLevel(level) =>{
                            //app_state.s
                            app_state.set(AppState::Fight);
                            println!("Ce niveau est de!");
                
                    }
                    ButtonAction::Quit => {
                        std::process::exit(0);
                    }
                    ButtonAction::Back => {
                        app_state.set(AppState::MainMenu);
                    }
                    ButtonAction::StartFight => {
                            println!("Début du combat !");
                            commands.insert_resource(NextState(AppState::Fight));
                            
                            // Démarrer le combat
                            commands.insert_resource(CombatState {
                                started: true,
                                finished: false,
                                turn: Turn::Player,  // Le joueur commence
                                ..Default::default()
                            });
                    }
                    ButtonAction::SelectSlot(slot_index) => {
                        selected_slot.slot = Some(*slot_index);
                    }
                    ButtonAction::ConfirmSlot => {
                        if selected_slot.slot.is_some() {
                            // Aller à l'écran de création de personnage plutôt qu'au jeu
                            app_state.set(AppState::PlayerCreation);
                        }
                    }
                    ButtonAction::CreatePlayer => {
                        // Créer le personnage et démarrer le jeu
                        let player = create_player(&PlayerCreationData::default()); // Idéalement, utiliser les vraies données
                        app_state.set(AppState::Levels);
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
