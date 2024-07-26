use std::collections::VecDeque;

use bevy::math::UVec2;
use bevy::prelude::*;
use bimap::{BiHashMap, Overwritten};
use pathfinding::directed::astar::astar;

use super::level::Terrain;

// On screen 0,0 is top middle tile,
// y increases left-down, x increases right-down
// Movement directions on tilemap
pub const NORTH: IVec2 = IVec2 { y: -1, x: 0 };
pub const EAST: IVec2 = IVec2::X;
pub const SOUTH: IVec2 = IVec2::Y;
pub const WEST: IVec2 = IVec2 { x: -1, y: 0 };
pub const NORTHEAST: IVec2 = NORTH.wrapping_add(EAST);
pub const SOUTHEAST: IVec2 = SOUTH.wrapping_add(EAST);
pub const NORTHWEST: IVec2 = NORTH.wrapping_add(WEST);
pub const SOUTHWEST: IVec2 = SOUTH.wrapping_add(WEST);

/// Four directional movement in straight lines like a rook
pub const ROOK_MOVES: [IVec2; 4] = [NORTH, EAST, SOUTH, WEST];

/// Eight directional movement like a king
pub const KING_MOVES: [IVec2; 8] = [
    NORTH, NORTHEAST, EAST, SOUTHEAST, SOUTH, SOUTHWEST, WEST, NORTHWEST,
];

#[derive(Resource, Default)]
pub struct VillageMap {
    pub size: UVec2,
    pub heat_map: Vec<u32>,
    pub ground: TileMap,
    pub object: TileMap,
}

impl VillageMap {
    pub fn new(size: UVec2) -> VillageMap {
        VillageMap {
            size,
            heat_map: Vec::new(),
            ground: TileMap::new(size.as_ivec2()),
            object: TileMap::new(size.as_ivec2()),
        }
    }

    pub fn isize(&self) -> IVec2 {
        self.size.as_ivec2()
    }

    pub fn is_out_of_bounds(&self, coord: IVec2) -> bool {
        coord.cmplt(IVec2::ZERO).any() || coord.cmpge(self.isize()).any()
    }

    /// Create a path from start to target while avoiding obstacles.
    pub fn pathfind(
        &self,
        start: &IVec2,
        target: &IVec2,
        directions: &[IVec2],
        is_airborne: bool,
        q_terrains: &Query<&Terrain>,
    ) -> Option<(Vec<IVec2>, i32)> {
        astar(
            start,
            // successors
            |tile_coord: &IVec2| {
                let tile_coord = *tile_coord;
                directions.iter().filter_map(move |m| {
                    let final_coord = tile_coord + *m;

                    let output = Some((final_coord, 1));
                    if self.is_out_of_bounds(final_coord) {
                        return None;
                    }

                    // There is an obstacle blocking it
                    if self.object.get(final_coord).is_some() {
                        return None;
                    }

                    // Check eligibility of moving on top of water tile
                    if let Some(terrain) = self
                        .ground
                        .get(final_coord)
                        .and_then(|e| q_terrains.get(e).ok())
                    {
                        match terrain {
                            Terrain::Water if is_airborne == false => return None,
                            _ => return output,
                        }
                    }

                    None
                })
            },
            // heuristic
            |tile_coord: &IVec2| IVec2::length_squared(target.wrapping_sub(*tile_coord)),
            // sucess
            |tile_coord: &IVec2| tile_coord == target,
        )
    }

    /// Generate heat map based on [`Self::object`].
    pub fn generate_heat_map(&mut self) {
        // Mark max as unvisted
        self.heat_map = vec![u32::MAX; (self.size.x * self.size.y) as usize];
        let mut stack = VecDeque::new();

        for tile_coord in self.object.map.left_values() {
            let index = (tile_coord.x + tile_coord.y * self.size.x as i32) as usize;
            self.heat_map[index] = 0;

            stack.push_back(*tile_coord);
        }

        while let Some(tile_coord) = stack.pop_front() {
            let index = (tile_coord.x + tile_coord.y * self.size.x as i32) as usize;
            let curr_heat = self.heat_map[index];

            for offset in KING_MOVES.iter() {
                let flood_coord = tile_coord.wrapping_add(*offset);
                if self.is_out_of_bounds(flood_coord) {
                    continue;
                }

                let index = (flood_coord.x + flood_coord.y * self.size.x as i32) as usize;

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
    size: IVec2,
    map: BiHashMap<IVec2, Entity>,
}

impl TileMap {
    pub fn new(size: IVec2) -> TileMap {
        assert!(IVec2::ZERO.cmplt(size).all());
        TileMap {
            size,
            map: BiHashMap::default(),
        }
    }

    pub fn size(&self) -> IVec2 {
        self.size
    }

    pub fn bounds(&self) -> IRect {
        IRect::from_corners(IVec2::ZERO, self.size - 1)
    }

    /// get entity at position
    pub fn get(&self, position: IVec2) -> Option<Entity> {
        self.map.get_by_left(&position).copied()
    }

    /// find entity's position in map
    pub fn locate(&self, entity: Entity) -> Option<IVec2> {
        self.map.get_by_right(&entity).copied()
    }

    /// place entity at map position, will move entity if already in map.
    /// will overwrite any existing entity at the position
    pub fn set(&mut self, position: IVec2, entity: Entity) -> Overwritten<IVec2, Entity> {
        self.map.insert(position, entity)
    }

    /// remove entity from map at position
    pub fn remove(&mut self, position: IVec2) -> Option<Entity> {
        self.map.remove_by_left(&position).map(|(_, entity)| entity)
    }

    /// remove entity from map
    pub fn remove_entity(&mut self, entity: Entity) -> Option<IVec2> {
        self.map
            .remove_by_right(&entity)
            .map(|(position, _)| position)
    }

    pub fn get_neighbouring_positions_rook(
        &self,
        position: IVec2,
    ) -> impl Iterator<Item = IVec2> + '_ {
        ROOK_MOVES
            .iter()
            .copied()
            .map(move |translation| position + translation)
            .filter(|target| self.bounds().contains(*target))
    }

    pub fn get_neighbouring_positions_king(
        &self,
        position: IVec2,
    ) -> impl Iterator<Item = IVec2> + '_ {
        KING_MOVES
            .iter()
            .copied()
            .map(move |translation| position + translation)
            .filter(|target| self.bounds().contains(*target))
    }
}
