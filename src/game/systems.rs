use bevy::prelude::*;

use super::unit::UnitTurnState;

pub fn reset_unit_states(mut unit_state_query: Query<&mut UnitTurnState>) {
    for mut unit_state in unit_state_query.iter_mut() {
        unit_state.reset();
    }
}
