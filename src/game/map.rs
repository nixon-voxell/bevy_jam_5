use std::collections::VecDeque;
use std::sync::TryLockError;

use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy::{math::UVec2, utils::HashMap};
use bimap::{BiHashMap, Overwritten};
use pathfinding::directed::astar::astar;

use crate::path_finding::find_all_within_distance_unweighted;
use crate::path_finding::map_position::{Tile, TileDim, TileRect, TileStep};

use super::{level::Terrain, unit::EnemyUnit};

// On screen 0,0 is top middle tile,
// y increases left-down, x increases right-down
// Movement directions on tilemap
// pub const NORTH: Pos = Pos { y: -1, x: 0 };
// pub const EAST: Pos = Pos::X;
// pub const SOUTH: Pos = Pos::Y;
// pub const WEST: Pos = Pos { x: -1, y: 0 };
// pub const NORTHEAST: Pos = NORTH.saturating_add(EAST);
// pub const SOUTHEAST: Pos = SOUTH.saturating_add(EAST);
// pub const NORTHWEST: Pos = NORTH.saturating_add(WEST);
// pub const SOUTHWEST: Pos = SOUTH.saturating_add(WEST);

/// Four directional movement in straight lines like a rook
// pub const TileStep::ROOK: [Pos; 4] = [NORTH, EAST, SOUTH, WEST];

// /// Eight directional movement like a king
// pub const TileStep::ALL: [Pos; 8] = [
//     NORTH, NORTHEAST, EAST, SOUTHEAST, SOUTH, SOUTHWEST, WEST, NORTHWEST,
// ];

#[derive(Resource, Default)]
pub struct VillageMap {
    pub size: TileDim,
    pub heat_map: Vec<u32>,
    pub terrain: HashMap<Tile, Terrain>,
    pub object: TileMap,
    pub deployment_zone: HashSet<Tile>,
}

impl VillageMap {
    pub fn new(size: TileDim) -> VillageMap {
        VillageMap {
            size,
            heat_map: Vec::new(),
            terrain: Default::default(),
            object: TileMap::new(size),
            deployment_zone: HashSet::default(),
        }
    }

    pub fn bounds(&self) -> TileRect {
        TileRect(Tile::ZERO, Tile(self.size.x(), self.size.y()))
    }

    pub fn size(&self) -> TileDim {
        self.size
    }

    pub fn is_out_of_bounds(&self, coord: Tile) -> bool {
        !self.bounds().contains(coord)
    }

    pub fn get_terrain(&self, coord: Tile) -> Option<Terrain> {
        self.terrain.get(&coord).copied()
    }

    pub fn set_terrain(&mut self, coord: Tile, terrain: Terrain) -> Option<Terrain> {
        self.terrain.insert(coord, terrain)
    }

    /// Create a path from start to target while avoiding obstacles.
    pub fn pathfind(
        &self,
        start: &Tile,
        target: &Tile,
        directions: &[TileStep],
        is_airborne: bool,
    ) -> Option<(Vec<Tile>, i32)> {
        astar(
            start,
            // successors
            |tile_coord: &Tile| {
                let tile_coord = *tile_coord;
                directions.iter().filter_map(move |dir| {
                    let final_coord = tile_coord.step(*dir);

                    if self.is_out_of_bounds(final_coord) {
                        return None;
                    }

                    // There is an obstacle blocking it
                    if self.object.get(final_coord).is_some() {
                        return None;
                    }

                    // Check eligibility of moving on top of water tile
                    if let Some(terrain) = self.get_terrain(final_coord) {
                        match terrain {
                            Terrain::Water if is_airborne == false => return None,
                            _ => return Some((final_coord, 1)),
                        }
                    }

                    None
                })
            },
            // heuristic
            |tile_coord: &Tile| tile_coord.distance_squared(*target),
            // sucess
            |tile_coord: &Tile| tile_coord == target,
        )
    }

    /// Flood into tiles within the range taking into consideration
    /// on terrain, obstacles, and directions.
    pub fn flood(
        &self,
        start: Tile,
        max_distance: u32,
        directions: &[TileStep],
        is_airborne: bool,
    ) -> HashSet<Tile> {
        find_all_within_distance_unweighted(start, max_distance, |tile_coord| {
            directions.iter().filter_map(move |dir| {
                let final_coord = tile_coord.step(*dir);

                if self.is_out_of_bounds(final_coord) {
                    return None;
                }

                // There is an obstacle blocking it
                if self.object.is_occupied(final_coord) {
                    return None;
                }

                // Check eligibility of moving on top of water tile
                if let Some(terrain) = self.get_terrain(final_coord) {
                    match terrain {
                        Terrain::Water if is_airborne == false => return None,
                        _ => return Some(final_coord),
                    }
                }

                None
            })
        })
    }

    /// Sort tiles based on distance.
    pub fn sort_tiles_by_distance(tiles: &mut [Tile], target_tile: Tile) {
        tiles.sort_by_key(|t| Tile::distance_squared(*t, target_tile));
    }

    /// Sort tiles based on heat map.
    pub fn sort_tiles_by_heat(&self, tiles: &mut [Tile]) {
        tiles.sort_by_key(|t| {
            let index = t.x() + t.y() * self.size.x() as i32;
            self.heat_map[index as usize]
        });
    }

    /// Get best tile based on heat map.
    pub fn get_best_tile(
        &self,
        start: Tile,
        max_distance: u32,
        directions: &[TileStep],
        is_airborne: bool,
    ) -> Option<Tile> {
        let mut tiles = self
            .flood(start, max_distance, directions, is_airborne)
            .iter()
            .cloned()
            .collect::<Vec<_>>();
        Self::sort_tiles_by_distance(&mut tiles, start);
        self.sort_tiles_by_heat(&mut tiles);
        tiles.first().copied()
    }

    /// Generate heat map based on [`Self::object`].
    ///
    /// # Example
    ///
    /// 4, 3, 2, 3, 4, 5, 6, 7, 8, 9,
    /// 3, 2, 1, 2, 3, 4, 5, 6, 7, 8,
    /// 2, 1, 0, 1, 2, 3, 4, 5, 6, 7,
    /// 2, 1, 1, 2, 2, 3, 4, 5, 6, 7,
    /// 1, 0, 1, 2, 1, 2, 3, 4, 5, 6,
    /// 2, 1, 2, 1, 0, 1, 2, 3, 4, 5,
    /// 3, 2, 3, 2, 1, 2, 3, 4, 5, 6,
    /// 4, 3, 4, 3, 2, 3, 4, 5, 6, 7,
    /// 5, 4, 5, 4, 3, 4, 5, 6, 7, 8,
    /// 6, 5, 6, 5, 4, 5, 6, 7, 8, 9,
    pub fn generate_heat_map(&mut self, is_enemy: impl Fn(Entity) -> bool) {
        // Mark max as unvisted
        self.heat_map = vec![u32::MAX; (self.size.x() * self.size.y()) as usize];
        let mut stack = VecDeque::new();

        for (tile_coord, entity) in self.object.map.iter() {
            if is_enemy(*entity) {
                continue;
            }

            let index = (tile_coord.x() + tile_coord.y() * self.size.x() as i32) as usize;
            self.heat_map[index] = 0;

            stack.push_back(*tile_coord);
        }

        if stack.is_empty() {
            self.heat_map.fill(0);
            return;
        }

        while let Some(tile_coord) = stack.pop_front() {
            let index = (tile_coord.x() + tile_coord.y() * self.size.x() as i32) as usize;
            let curr_heat = self.heat_map[index];

            for offset in TileStep::ROOK.iter() {
                let flood_coord = tile_coord.step(*offset);
                if self.is_out_of_bounds(flood_coord) {
                    continue;
                }

                let index = (flood_coord.x() + flood_coord.y() * self.size.x() as i32) as usize;

                // Has been visited
                if self.heat_map[index] != u32::MAX {
                    continue;
                }

                self.heat_map[index] = curr_heat + 1;
                stack.push_back(flood_coord);
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct TileMap {
    size: TileDim,
    map: BiHashMap<Tile, Entity>,
}

impl TileMap {
    pub fn new(size: TileDim) -> TileMap {
        assert!(Tile::ZERO.to_ivec2().cmplt(size.to_ivec2()).all());
        TileMap {
            size,
            map: BiHashMap::default(),
        }
    }

    pub fn size(&self) -> TileDim {
        self.size
    }

    pub fn bounds(&self) -> TileRect {
        TileRect(Tile::ZERO, Tile(self.size.x() - 1, self.size.y() - 1))
    }

    pub fn is_occupied(&self, position: Tile) -> bool {
        self.map.get_by_left(&position).is_some()
    }

    /// get entity at position
    pub fn get(&self, position: Tile) -> Option<Entity> {
        self.map.get_by_left(&position).copied()
    }

    /// find entity's position in map
    pub fn locate(&self, entity: Entity) -> Option<Tile> {
        self.map.get_by_right(&entity).copied()
    }

    /// place entity at map position, will move entity if already in map.
    /// will overwrite any existing entity at the position
    pub fn set(&mut self, position: Tile, entity: Entity) -> Overwritten<Tile, Entity> {
        self.map.insert(position, entity)
    }

    /// remove entity from map at position
    pub fn remove(&mut self, position: Tile) -> Option<Entity> {
        self.map.remove_by_left(&position).map(|(_, entity)| entity)
    }

    /// remove entity from map
    pub fn remove_entity(&mut self, entity: Entity) -> Option<Tile> {
        self.map
            .remove_by_right(&entity)
            .map(|(position, _)| position)
    }

    pub fn get_neighbouring_positions_rook(
        &self,
        position: Tile,
    ) -> impl Iterator<Item = Tile> + '_ {
        TileStep::ROOK
            .iter()
            .copied()
            .map(move |translation| position.step(translation))
            .filter(|target| self.bounds().contains(*target))
    }

    pub fn get_neighbouring_positions_king(
        &self,
        position: Tile,
    ) -> impl Iterator<Item = Tile> + '_ {
        TileStep::ALL
            .iter()
            .copied()
            .map(move |translation| position.step(translation))
            .filter(|target| self.bounds().contains(*target))
    }
}
