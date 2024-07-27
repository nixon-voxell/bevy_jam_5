use bevy::prelude::*;

use crate::game::unit_list::PlayerUnitList;

use super::*;

pub const INITIAL_PLAYER_UNITS: usize = 2;
pub const MAX_PLAYER_UNITS: usize = 5;

#[derive(Component, Default)]
pub struct PlayerUnit;

pub fn add_starting_player_units(
    mut available_names: ResMut<AvailableUnitNames>,
    mut player_unit_list: ResMut<PlayerUnitList>,
    mut commands: Commands,
) {
    player_unit_list.0.clear();
    for _ in 0..INITIAL_PLAYER_UNITS {
        let name = available_names.next_name();
        println!("add name: {name}");
        let id = commands
            .spawn(
                UnitBundle::<PlayerUnit>::new(&name)
                    .with_health(3)
                    .with_hit_points(3)
                    .with_movement(3),
            )
            .id();
        player_unit_list.0.push(id);
    }
}
