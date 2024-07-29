use bevy::prelude::*;

use crate::game::cycle::EndTurn;
use crate::game::inventory::Inventory;
use crate::game::map::{VillageMap, ROOK_MOVES};
use crate::game::picking::TilePressedEvent;
use crate::game::selection::SelectedUnit;
use crate::game::tile_set::tile_coord_translation;
use crate::game::unit_list::PlayerUnitList;
use crate::path_finding::find_all_within_distance_unweighted;
use crate::screen::playing::GameState;

use super::*;

pub const INITIAL_PLAYER_UNITS: usize = 2;
pub const MAX_PLAYER_UNITS: usize = 5;

pub fn add_starting_player_units(
    mut available_names: ResMut<AvailableUnitNames>,
    mut player_unit_list: ResMut<PlayerUnitList>,
    mut commands: Commands,
) {
    player_unit_list.0.clear();
    for _ in 0..INITIAL_PLAYER_UNITS {
        let name = available_names.next_name();
        let id = commands
            .spawn((
                SpatialBundle {
                    visibility: Visibility::Hidden,
                    ..default()
                },
                UnitBundle::<PlayerUnit>::new(&name, ROOK_MOVES.to_vec())
                    .with_health(3)
                    .with_hit_points(3)
                    .with_movement(3),
                Inventory::default(),
            ))
            .id();
        player_unit_list.0.push(id);
        println!("add name: {name} ({id})");
    }
}

pub fn move_unit(
    mut event_reader: EventReader<TilePressedEvent>,
    selected_unit: Res<SelectedUnit>,
    mut map: ResMut<VillageMap>,
    mut turn_state_query: Query<(&mut UnitTurnState, &Movement), With<PlayerUnit>>,
    mut transform: Query<&mut Transform>,
) {
    if let Some(TilePressedEvent(target)) = event_reader.read().last() {
        println!("tile pressed -> {target:?}");
        let Some(selected) = selected_unit.entity else {
            return;
        };

        println!("selected -> {selected:?}");
        let Ok((mut turn_state, movement)) = turn_state_query.get_mut(selected) else {
            return;
        };
        println!("turn state -> {turn_state:?}");

        if turn_state.used_move || movement.0 == 0 {
            return;
        }

        let Some(current_pos) = map.object.locate(selected).filter(|pos| *pos != *target) else {
            return;
        };

        println!("current_pose = {current_pos}");

        if find_all_within_distance_unweighted(current_pos, movement.0, |t| {
            map.terrain
                .get_neighbouring_positions_rook(t)
                .filter(|n| map.object.get(*n).is_none())
        })
        .contains(target)
        {
            println!("move {selected} to {target:?}");
            map.object.set(*target, selected);
            turn_state.used_move = true;
            transform.get_mut(selected).unwrap().translation =
                tile_coord_translation(target.x as f32, target.y as f32, 2.);
        }
    }
}

pub fn reset_unit_turn_states(
    mut events: EventReader<EndTurn>,
    mut query: Query<&mut UnitTurnState>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for _ in events.read() {
        for mut turn_state in query.iter_mut() {
            turn_state.reset();
            next_game_state.set(GameState::EnemyTurn);
        }
    }
}
