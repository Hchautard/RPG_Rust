use bevy::prelude::*;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect};
use crate::services::ui::constants::NORMAL_BUTTON;
use crate::services::ui::game::{GameScreen, GameButtonAction, GameScreenState, BouncerQuestionUI};

pub fn spawn_bouncer_question_screen(commands: &mut Commands, game_state: &GameScreenState) {
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