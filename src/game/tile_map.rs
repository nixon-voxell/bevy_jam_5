use bevy::prelude::*;
use bimap::BiMap;
use bimap::Overwritten;


pub struct TileMap {
    size: UVec2,
    map: BiMap<UVec2, Entity>,
}

impl TileMap {
    pub fn new(size: UVec2) -> TileMap {
        assert!(UVec2::ZERO.cmplt(size).all());
        TileMap {
            size,
            map: BiMap::default()
        }
    }

    pub fn get(&self, position: UVec2) -> Option<Entity> {
        self.map.get_by_left(&position).copied()
    }

    pub fn locate(&self, entity: Entity) -> Option<UVec2> {
        self.map.get_by_right(&entity).copied()
    }

    pub fn set(&mut self, position: UVec2, entity: Entity) -> Overwritten<UVec2, Entity>{
        self.map.insert(position, entity)
    }

    pub fn remove(&mut self, position: UVec2) -> Option<Entity> {
        self.map.remove_by_left(&position).map(|(_, entity)| entity)
    }

    pub fn remove_entity(&mut self, entity: Entity) -> Option<UVec2> {
        self.map.remove_by_right(&entity).map(|(position, _)| position)
    }
}