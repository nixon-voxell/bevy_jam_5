use bevy::prelude::*;

use super::components::UnitState;

pub fn reset_unit_states(
    mut unit_state_query: Query<&mut UnitState>,
) {
    for mut unit_state in unit_state_query.iter_mut() {
        unit_state.reset();
    }
}