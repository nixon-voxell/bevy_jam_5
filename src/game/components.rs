use bevy::prelude::*;
use bevy::math::UVec2;


/// amount of damage a unit can take before dying
#[derive(Component, Copy, Clone, Debug, Deref, DerefMut, PartialEq)]
pub struct HitPoints(pub u32);

/// number of tiles a unit can move per turn
#[derive(Component, Copy, Clone, Debug, Deref, DerefMut, PartialEq)]
pub struct Movement(pub u32);

/// Name of the unit
#[derive(Component)]
pub struct UnitName(pub String);

/// Is the unit player controlled
#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub enum UnitAllegiance {
    Player,
    Enemy
}

/// Grid position of the unit
#[derive(Component, Copy, Clone, Debug, Deref, DerefMut, PartialEq)]
pub struct Position(pub UVec2);

/// Has unit moved or performed an action yet
/// Needs to be reset to default after each turn (Not good?)
#[derive(Component, Default)]
pub struct UnitState {
    pub used_move: bool,
    pub used_action: bool,
}




