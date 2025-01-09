use bevy::prelude::*;

use crate::card::{card::Card, Deck};

use super::{game_manager::GameManager, playing_field::PlayingField};

// Removes every Entity related to a running game
pub(crate) fn game_cleanup(
    mut commands: Commands,
    mut cards: Query<Entity, With<Card>>,
    mut game_manager: Query<Entity, With<GameManager>>,
    mut playing_field: Query<Entity, With<PlayingField>>,
    mut deck: Query<Entity, With<Deck>>,
) {
    for card in &mut cards {
        commands.entity(card).despawn_recursive();
    }

    for gm in &mut game_manager {
        commands.entity(gm).despawn_recursive();
    }

    for pf in &mut playing_field {
        commands.entity(pf).despawn_recursive();
    }

    for dk in &mut deck {
        commands.entity(dk).despawn_recursive();
    }
}
