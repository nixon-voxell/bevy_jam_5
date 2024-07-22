use bevy::math::UVec2;
use bevy::prelude::*;

use super::tile_map::TileMap;

#[derive(Resource, Debug, Copy, Clone, PartialEq)]
pub struct Turn(pub u32);

#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub enum TurnPhase {
    #[default]
    Player,
    Enemy,
}

#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub enum Season {
    #[default]
    Summer,
    Autumn,
    Winter,
}

#[derive(Resource, Debug, Copy, Clone, PartialEq)]
pub struct DayCycle {
    /// Number of daylight turns in a day
    pub day: u32,
    /// Number of night turns in a day
    pub night: u32,
}

impl Default for DayCycle {
    fn default() -> Self {
        Self::from(Season::Summer)
    }
}

impl From<Season> for DayCycle {
    fn from(season: Season) -> Self {
        match season {
            Season::Summer => DayCycle {
                day: 6,
                night: 4,
            },
            Season::Autumn => DayCycle {
                day: 5,
                night: 5,
            },
            Season::Winter => DayCycle {
                day: 4,
                night: 6,
            },
        }
    }
}

#[derive(Resource, Debug, Copy, Clone, PartialEq)]
pub struct DaysUntilFullMoon(pub u32);

impl Default for DaysUntilFullMoon {
    fn default() -> Self {
        Self(3)
    }
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
