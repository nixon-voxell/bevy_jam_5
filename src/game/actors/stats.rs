//! Components representing common properties of game actors

use bevy::prelude::*;

/// Amount of armour the actor has
/// Takes damage first instead of health
/// Lost armour points aren't regained by healing potions
pub struct Armour(pub u32);

/// Amount of health the actor has.
/// When health drops to 0 the unit is destroyed
#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct Health {
    pub value: u32,
    pub max: u32,
}

impl Health {
    pub fn new(value: u32) -> Self {
        Self { value, max: value }
    }

    pub fn is_full(self) -> bool {
        self.value == self.max
    }

    pub fn is_empty(self) -> bool {
        self.value == 0
    }

    pub fn heal(mut self, value: u32) -> u32 {
        self.value = (self.value + value).min(self.max);
        self.value
    }

    pub fn hurt(mut self, value: u32) -> u32 {
        self.value = self.value.saturating_sub(value);
        self.value
    }
}

/// Number of tiles an actor can move per turn.
#[derive(Component, Copy, Clone, Debug, Deref, DerefMut, PartialEq)]
pub struct Movement(pub u32);

// pub struct StatsBundle {
//     health: Health,
//     movement: Movement,
// }

/// Name of the actor, used to identify it to the player
/// Does not have to be unique
#[derive(Component, Default, PartialEq, Debug)]
pub struct ActorName(pub String);
