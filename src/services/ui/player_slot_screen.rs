use bevy::prelude::*;
use crate::services::ui::constants::{AppState, ButtonAction, NORMAL_BUTTON, SELECTED_BUTTON, WHITE, BLACK, BLUE};

/// Composant pour marquer les entités de l'écran de sélection de slot
#[derive(Component)]
pub struct PlayerSlotScreen;

/// Ressource pour stocker le slot de joueur sélectionné
#[derive(Resource)]
pub struct SelectedPlayerSlot {
    pub slot: Option<usize>,
}

/// Plugin pour l'écran de sélection de slot
pub struct PlayerSlotScreenPlugin;

impl Plugin for PlayerSlotScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SelectedPlayerSlot>()
            .add_systems(OnEnter(AppState::PlayerSlot), setup_player_slot_screen)
            .add_systems(OnExit(AppState::PlayerSlot), despawn_player_slot_screen)
            .add_systems(Update, update_slot_selection);
    }
}

impl Default for SelectedPlayerSlot {
    fn default() -> Self {
        Self { slot: None }
    }
}

/// Système pour initialiser l'écran de sélection de slot
pub fn setup_player_slot_screen(mut commands: Commands) {
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
        // Titre de l'écran
        parent.spawn(Text::new("Choisissez un slot de sauvegarde"));

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
                        
                        // On pourrait ajouter ici des informations sur le slot
                        button.spawn(Text::new("Vide"));
                    });
            }
        });

        // Bouton de confirmation
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
                button.spawn(Text::new("Confirmer"));
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
    if selected_slot.is_changed() {
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