use bevy::math::UVec2;
use bevy::prelude::*;

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

pub struct TileMap {
    size: UVec2,
    tiles: Vec<Entity>,
}

impl TileMap {
    pub fn new(size: UVec2) -> TileMap {
        assert!(UVec2::ZERO.cmplt(size).all());
        let tile_count = (size.x * size.y) as usize;
        TileMap {
            size,
            tiles: vec![Entity::PLACEHOLDER; tile_count],
        }
    }

    pub fn get_index(&self, position: UVec2) -> Option<usize> {
        position.cmplt(self.size).all().then_some(
            (position.y * self.size.x + position.x) as usize
        )
    }

    pub fn get(&self, position: UVec2) -> Option<Entity> {
        self.get_index(position)
            .map(|index| self.tiles[index])
            .filter(|entity| *entity != Entity::PLACEHOLDER)
    }

    pub fn set(&mut self, position: UVec2, mut entity: Entity) -> Option<Entity> {
        self.get_index(position)
            .and_then(|index| 
                self.tiles.get_mut(index)
                    .replace(&mut entity)
                    .copied()
            )
    }

    pub fn take(&mut self, position: UVec2) -> Option<Entity> {
        self.get_index(position)
            .map(|index| 
                std::mem::replace(&mut self.tiles[index], Entity::PLACEHOLDER)
            )
            .filter(|entity| *entity != Entity::PLACEHOLDER)
    }
}