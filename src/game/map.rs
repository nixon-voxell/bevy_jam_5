use bevy::math::UVec2;
use bevy::prelude::*;

use super::tile_map::TileMap;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

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
            foreground: TileMap::new(size.as_ivec2()),
            background: TileMap::new(size.as_ivec2()),
        }
    }
}
