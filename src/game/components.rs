use bevy::prelude::*;
use bevy::math::UVec2;
use bevy::utils::HashSet;


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
#[derive(Component, Default, Debug)]
pub struct UnitState {
    pub used_move: bool,
    pub used_action: bool,
}

impl UnitState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Component, Default, Debug, Copy, Clone)]
pub enum Terrain {
    #[default]
    /// Tile is grassland or road,
    Clear,
    Water,
}

#[derive(Debug, Copy, Clone)]
pub enum Ability {
    /// Intagible units can move through all other units, walls and buildings. 
    Intangible,
    /// Can move through all terrain
    /// Can move diagonally    
    Flying,

    /// `Aquatic`` units can move like `Swimmer`s
    /// but can also end their turn on a water tile and take actions while on a water tile.
    Aquatic,

    /// `Swimmer`s move through a water tile. They can't end a turn on a water tile however.
    Swimmer,

    /// Can move a second time per turn instead of taking an action.
    Runner,

    /// Can perform two actions if they don't move this turn.
    Operator,

    /// Can move after performing an action, if they hadn't already moved this turn.
    Tactician
}

/// The abilities could be individual marker components but storing them together in a hashmap
/// seems more manageable.
#[derive(Component, Default, Debug, Clone)]
pub struct Abilites(pub HashSet<Ability>);

/// Marker component for a building
#[derive(Component)]
pub struct Structure;

