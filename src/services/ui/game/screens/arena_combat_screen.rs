use bevy::prelude::*;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect};
use crate::services::ui::constants::NORMAL_BUTTON;
use crate::services::ui::game::{GameScreen, GameButtonAction, GameScreenState, ArenaUI};
use std::collections::HashSet;

/// Affiche l'écran de combat d'Arène.
/// Cet écran permet de combattre un Maître d'Arène en sélectionnant des ingrédients pour concocter un cocktail.
/// Il affiche les HP du joueur et du boss, ainsi que les ingrédients sélectionnés.
/// # Arguments
/// - `commands`: Les commandes pour créer des entités dans Bevy.
/// - `game_state`: L'état du jeu contenant les informations nécessaires pour l'écran de combat.
pub fn spawn_arena_combat_screen(commands: &mut Commands, game_state: &GameScreenState) {
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
        if game_state.show_intro_screen {
            spawn_intro_content(parent, game_state);
            return;
        } else if game_state.show_crafting_phase {
            spawn_crafting_phase_content(parent);
            return;
        }

        spawn_combat_content(parent, game_state);
    });
}

fn spawn_intro_content(parent: &mut ChildBuilder, game_state: &GameScreenState) {
    // Titre
    parent.spawn(Text::new(format!(
        "Vous allez affronter {} sur {}",
        game_state.master_name.as_deref().unwrap_or("???"),
        game_state.selected_arena.as_deref().unwrap_or("???"),
    )));

    // Bouton "Commencer le combat"
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
            GameButtonAction::BackToMainFromCombat,
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
        ))
        .with_child(Text::new("Retour"));
}

fn spawn_crafting_phase_content(parent: &mut ChildBuilder) {
    parent.spawn(Text::new(
        "🎉 Bien joué ! Tu as trouvé la bonne recette.\nMaintenant concocte le cocktail comme il faut pour finir le boss."
    ));

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
            GameButtonAction::StartFinalCraft,
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
        ))
        .with_child(Text::new("Continuer"));
}

fn spawn_combat_content(parent: &mut ChildBuilder, game_state: &GameScreenState) {
    // Titre du combat
    if let Some(master_name) = &game_state.master_name {
        parent.spawn(Text::new(format!("Combat contre le Maître: {}", master_name)));
    } else {
        parent.spawn(Text::new("Combat d'Arène"));
    }

    // HP
    parent.spawn(Text::new(format!("Votre HP: {}", game_state.player_hp)));
    parent.spawn(Text::new(format!("HP du Boss: {}", game_state.boss_hp)));

    // Section des ingrédients
    spawn_ingredient_selection(parent, game_state);

    // Validation du cocktail
    spawn_cocktail_validation(parent, game_state);

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
            GameButtonAction::BackToMainFromCombat,
            BorderColor(Color::BLACK),
            BorderRadius::MAX,
            BackgroundColor(NORMAL_BUTTON),
        ))
        .with_child(Text::new("Retour"));
}

fn spawn_ingredient_selection(parent: &mut ChildBuilder, game_state: &GameScreenState) {
    parent.spawn(Text::new("Sélectionnez les ingrédients pour le cocktail:"));

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

    if let Some(recipe) = &game_state.master_recipe {
        for ingredient in &recipe.ingredients {
            all_ingredients_set.insert(ingredient.name.clone());
        }
    }

    let mut all_ingredients: Vec<String> = all_ingredients_set.into_iter().collect();
    all_ingredients.sort();

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

    parent.spawn(Text::new(format!(
        "Ingrédients sélectionnés: {:?}",
        game_state.current_crafting.selected_ingredients
    )));
}

fn spawn_cocktail_validation(parent: &mut ChildBuilder, game_state: &GameScreenState) {
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

    if let Some(recipe) = &game_state.master_recipe {
        let selected = &game_state.current_crafting.selected_ingredients;
        let expected: HashSet<String> = recipe.ingredients.iter().map(|i| i.name.clone()).collect();

        let correct_count = selected.iter().filter(|i| expected.contains(*i)).count();
        let incorrect_count = selected.len() - correct_count;

        let is_valid = selected.len() == expected.len() && incorrect_count == 0;

        let validation_text = if is_valid {
            "✅ Cocktail valide !".to_string()
        } else {
            format!("❌ Cocktail incorrect : {} bon(s), {} mauvais.", correct_count, incorrect_count)
        };

        parent.spawn(Text::new(validation_text));
    }
}