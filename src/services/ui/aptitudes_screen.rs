use bevy::prelude::*;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect};
use crate::models::aptitude::Aptitude;
use crate::services::ui::constants::{ButtonAction, NORMAL_BUTTON};

#[derive(Component)]
pub struct AptitudesScreen;

#[derive(Resource)]
pub struct AptitudeList {
    pub aptitudes: Vec<Aptitude>,
}

pub fn setup_aptitudes_screen(mut commands: Commands, aptitude_list: Res<AptitudeList>) {
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

pub fn despawn_aptitudes_screen(mut commands: Commands, query: Query<Entity, With<AptitudesScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}