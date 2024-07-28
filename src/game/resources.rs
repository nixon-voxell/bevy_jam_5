use bevy::prelude::*;

use super::constants::INITIAL_GOLD;
use super::constants::INITIAL_POPULATION;

#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub enum TurnPhase {
    #[default]
    Player,
    Enemy,
}

#[derive(Resource, Debug, Copy, Clone, PartialEq)]
pub struct VillageGold(pub u32);

impl Default for VillageGold {
    fn default() -> Self {
        Self(INITIAL_GOLD)
    }
}

impl std::fmt::Display for VillageGold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Resource, Debug, Copy, Clone, PartialEq)]
pub struct VillagePopulation(pub u32);

impl Default for VillagePopulation {
    fn default() -> Self {
        Self(INITIAL_POPULATION)
    }
}

impl std::fmt::Display for VillagePopulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
