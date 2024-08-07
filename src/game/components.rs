use bevy::prelude::*;
use bevy::utils::HashSet;

#[derive(Debug, Copy, Clone)]
pub enum Ability {
    /// Intagible actors can move through all other actors, walls and buildings.
    Intangible,

    /// Can move through all terrain
    /// Can move diagonally    
    Flying,

    /// `Aquatic`` actors can move like `Swimmer`s
    /// but can also end their turn on a water tile and take actions while on a water tile.
    Aquatic,

    /// `Swimmer`s move through a water tile. They can't end a turn on a water tile however.
    Swimmer,

    /// Can move a second time per turn instead of taking an action.
    Runner,

    /// Can perform two actions if they don't move this turn.
    Operator,

    /// Can move after performing an action, if they hadn't already moved this turn.
    Tactician,
}

/// The abilities could be individual marker components but storing them together in a hashmap
/// seems more manageable.
#[derive(Component, Default, Debug, Clone)]
pub struct Abilities(pub HashSet<Ability>);

#[derive(Component, Debug)]
pub struct GroundTileLayer;

#[derive(Component, Debug)]
pub struct ActorTileLayer;

#[derive(Component, Debug)]
pub struct Income(pub u32);

#[derive(Component, Debug)]
pub struct PopulationCapacity(pub u32);

#[derive(Component, Debug)]
pub struct RemainingConstructionTurns(pub u32);

#[derive(Component)]
pub struct BuildingProgressLabel;

#[derive(Component)]
pub struct ConstructionWorkers(pub u32);

#[derive(Component)]
pub struct Tavern;
#[derive(Component)]
pub struct House;

#[derive(Component)]
pub struct Blacksmith;

#[derive(Component)]
pub struct ArcherTower;
