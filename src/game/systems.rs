use bevy::prelude::*;

use super::actors::ActorTurnState;

pub fn reset_unit_states(mut unit_state_query: Query<&mut ActorTurnState>) {
    for mut unit_state in unit_state_query.iter_mut() {
        unit_state.reset();
    }
}
