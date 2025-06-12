use bevy::prelude::*;
use bevy::ui::{Val, JustifyContent, AlignItems, FlexDirection, UiRect};
use crate::services::ui::constants::NORMAL_BUTTON;
use crate::services::ui::game::{GameScreen, GameButtonAction, GameScreenState, ArenaUI, ArenaCombatState};

/// Affiche l'écran de fin d'Arene.
/// Cet écran affiche différents messages selon que le joueur ait gagné ou perdu.
/// # Arguments
/// - `commands`: Les commandes pour créer des entités dans Bevy.
/// - `game_state`: L'état du jeu contenant les informations nécessaires pour l'écran de fin d'Arene.
pub fn spawn_arena_end_screen(commands: &mut Commands, game_state: &GameScreenState) {
    // Déterminer si le joueur a gagné ou perdu
    let player_won = match game_state.arena_combat_state {
        ArenaCombatState::Victory => true,
        ArenaCombatState::Defeat => false,
        _ => game_state.boss_hp == 0 && game_state.player_hp > 0,
    };
    
    let player_lost = match game_state.arena_combat_state {
        ArenaCombatState::Defeat => true,
        ArenaCombatState::Victory => false,
        _ => game_state.player_hp == 0,
    };
    
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
        // Afficher le message approprié selon le résultat
        if player_won {
            parent.spawn(Text::new("Bravo ! Vous avez battu le boss !"));
            
            parent.spawn(Text::new(format!(
                "Maitre battu : {}\nArene : {}",
                game_state.master_name.as_deref().unwrap_or("???"),
                game_state.selected_arena.as_deref().unwrap_or("???"),
            )));
            
            // Message selon la phase où la victoire a eu lieu
            if game_state.current_crafting.cocktail_ready {
                parent.spawn(Text::new("Vous avez parfaitement execute la recette !"));
            } else {
                parent.spawn(Text::new("Vous avez trouve la bonne combinaison d'ingredients !"));
            }
        } else if player_lost {
            parent.spawn(Text::new("Defaite ! Vous etes tombe au combat..."));
            
            parent.spawn(Text::new(format!(
                "Vous avez ete vaincu par {} dans l'arene {}.",
                game_state.master_name.as_deref().unwrap_or("???"),
                game_state.selected_arena.as_deref().unwrap_or("???"),
            )));
            
            // Message selon la phase où la défaite a eu lieu
            if game_state.show_crafting_phase || game_state.current_crafting.cocktail_ready {
                parent.spawn(Text::new("Vous avez echoue lors de l'execution de la recette."));
            } else {
                parent.spawn(Text::new("Vous n'avez pas reussi a trouver la bonne recette."));
            }
            
            parent.spawn(Text::new("Vos HP sont tombes a zero !"));
        } else {
            // Cas où le combat s'est terminé autrement
            parent.spawn(Text::new("Combat termine"));
            
            parent.spawn(Text::new(format!(
                "HP restants - Joueur: {} | Boss: {}",
                game_state.player_hp,
                game_state.boss_hp
            )));
        }

        // Afficher les HP finaux
        parent.spawn(Text::new(format!(
            "HP finaux - Vous: {} | Boss: {}",
            game_state.player_hp,
            game_state.boss_hp
        )));

        // Bouton pour retourner à la sélection des arènes
        parent
            .spawn((
                Button,
                Node {
                    width: Val::Px(300.0),
                    height: Val::Px(60.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                GameButtonAction::SelectArena,
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(NORMAL_BUTTON),
            ))
            .with_child(Text::new("Retour a la selection des niveaux"));
    });
}