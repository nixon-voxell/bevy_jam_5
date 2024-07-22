use bevy::math::UVec2;
use bevy::prelude::*;

use super::tile_map::TileMap;

#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub enum TurnPhase {
    #[default]
    Player,
    Enemy,
}

#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub struct PlayerGold(pub u32);

#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub struct VillagePopulation(pub u32);

#[derive(Resource)]
pub struct VillageMap {
    size: UVec2,
    pub foreground: TileMap,
    pub background: TileMap,
}

impl VillageMap {
    pub fn new(size: UVec2) -> VillageMap {
        VillageMap {
            size,
            foreground: TileMap::new(size),
            background: TileMap::new(size),
        }
    }
}
