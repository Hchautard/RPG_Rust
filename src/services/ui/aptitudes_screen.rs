use crate::models::aptitude::Aptitude;
use crate::services::ui::constants::{ButtonAction, NORMAL_BUTTON};
use bevy::prelude::*;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, UiRect, Val};

/// Marqueur de composant pour identifier les entités de l'écran des aptitudes
#[derive(Component)]
pub struct AptitudesScreen;

/// Ressource qui contient la liste des aptitudes à afficher
#[derive(Resource)]
pub struct AptitudeList {
    // Vecteur de toutes les aptitudes disponibles
    pub aptitudes: Vec<Aptitude>,
}

/// On crée l'interface d'affichage des aptitudes avec la liste complète
/// # Arguments
/// - `commands`: Les commandes pour créer des entités dans Bevy.
/// - `aptitude_list`: La liste des aptitudes à afficher, passée en tant que ressource.
pub fn setup_aptitudes_screen(mut commands: Commands, aptitude_list: Res<AptitudeList>) {
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
            AptitudesScreen,
        ))
        .with_children(|parent| {
            // Affichage de chaque aptitude

            for aptitude in &aptitude_list.aptitudes {
                parent.spawn(Text::from(aptitude.name.clone()));
            }

            // Bouton de retour au menu principal

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
                .with_child((Text::new("Retour"),));
        });
}

/// On nettoie l'écran des aptitudes en supprimant toutes les entités associées
pub fn despawn_aptitudes_screen(
    mut commands: Commands,
    query: Query<Entity, With<AptitudesScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
