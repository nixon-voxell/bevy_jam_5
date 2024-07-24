use bevy::prelude::*;

#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub enum TurnPhase {
    #[default]
    Player,
    Enemy,
}
