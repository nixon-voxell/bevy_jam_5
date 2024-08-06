use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::utils::HashSet;
use bimap::{BiHashMap, Overwritten};
use pathfinding::directed::astar::astar;
use std::collections::VecDeque;

use crate::path_finding::find_all_within_distance_unweighted;
use crate::path_finding::tiles::Tiled;
use crate::path_finding::tiles::{Tile, TileDim, TileDir, TileRect};

use super::level::Terrain;

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
    pub actors: TileMap,
    pub deployment_zone: HashSet<Tile>,
}

impl VillageMap {
    pub fn new(size: TileDim) -> VillageMap {
        VillageMap {
            size,
            heat_map: Vec::new(),
            terrain: Default::default(),
            actors: TileMap::new(size),
            deployment_zone: HashSet::default(),
        }
    }

    pub fn bounds(&self) -> TileRect {
        TileRect(Tile::ZERO, Tile(self.size.x() - 1, self.size.y() - 1))
    }

    pub fn size(&self) -> TileDim {
        self.size
    }

    pub fn is_out_of_bounds(&self, tile: Tile) -> bool {
        !self.bounds().contains(tile)
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
        directions: &[TileDir],
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
                    if self.actors.get(final_coord).is_some() {
                        return None;
                    }

                    // Check eligibility of moving on top of water tile
                    match self.get_terrain(final_coord) {
                        Some(Terrain::Water) if is_airborne == false => return None,
                        _ => return Some((final_coord, 1)),
                    }
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
        directions: &[TileDir],
        is_airborne: bool,
    ) -> HashSet<Tile> {
        find_all_within_distance_unweighted(start, max_distance, |tile_coord| {
            directions.iter().filter_map(move |dir| {
                let final_coord = tile_coord.step(*dir);
                if self.is_out_of_bounds(final_coord) {
                    return None;
                }

                // There is an obstacle blocking it
                if self.actors.is_occupied(final_coord) {
                    return None;
                }

                // Check eligibility of moving on top of water tile
                match self.get_terrain(final_coord) {
                    Some(Terrain::Water) if !is_airborne => return None,
                    _ => return Some(final_coord),
                }
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
        directions: &[TileDir],
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

    /// Get worst tile based on heat map.
    pub fn get_worst_tile(
        &self,
        start: Tile,
        max_distance: u32,
        directions: &[TileDir],
        is_airborne: bool,
    ) -> Option<Tile> {
        let mut tiles = self
            .flood(start, max_distance, directions, is_airborne)
            .iter()
            .cloned()
            .collect::<Vec<_>>();

        Self::sort_tiles_by_distance(&mut tiles, start);
        self.sort_tiles_by_heat(&mut tiles);
        tiles.last().copied()
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

        for (tile_coord, entity) in self.actors.map.iter() {
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

            for offset in TileDir::EDGES.iter() {
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

    pub fn iter_terrain(&self) -> impl Iterator<Item = (Tile, Terrain)> + '_ {
        self.terrain.iter().map(|(tile, ter)| (*tile, *ter))
    }
}

impl Tiled for VillageMap {
    fn contains_tile(&self, tile: Tile) -> bool {
        self.bounds().contains(tile)
    }

    fn find_perimeter(&self, directions: &[TileDir]) -> impl Iterator<Item = Tile> {
        let bounds = self.bounds();
        bounds.into_iter().filter(move |tile| {
            directions
                .iter()
                .all(|direction| bounds.contains(tile.step(*direction)))
        })
    }
}

#[derive(Debug, Default)]
pub struct TileMap {
    size: TileDim,
    map: BiHashMap<Tile, Entity>,
}

impl TileMap {
    pub fn new(size: TileDim) -> TileMap {
        assert!(0 < size.x() && 0 < size.y());
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
        TileDir::EDGES
            .iter()
            .copied()
            .map(move |translation| position.step(translation))
            .filter(|target| self.bounds().contains(*target))
    }

    pub fn get_neighbouring_positions_king(
        &self,
        position: Tile,
    ) -> impl Iterator<Item = Tile> + '_ {
        TileDir::ALL
            .iter()
            .copied()
            .map(move |translation| position.step(translation))
            .filter(|target| self.bounds().contains(*target))
    }

    pub fn iter(&self) -> impl Iterator<Item = (Tile, Entity)> + '_ {
        self.map.iter().map(|(t, e)| (*t, *e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_village_map_creation() {
        let size = TileDim(10, 10);
        let village_map = VillageMap::new(size);
        assert_eq!(village_map.size(), size);
        assert!(village_map.terrain.is_empty());
        assert!(village_map.deployment_zone.is_empty());
    }

    #[test]
    fn test_village_map_bounds() {
        let size = TileDim(10, 10);
        let village_map = VillageMap::new(size);
        let bounds = village_map.bounds();
        assert_eq!(bounds.min(), Tile::ZERO);
        assert_eq!(bounds.max(), Tile(9, 9));
    }

    #[test]
    fn test_village_map_is_out_of_bounds() {
        let size = TileDim(10, 10);
        let village_map = VillageMap::new(size);
        assert!(village_map.is_out_of_bounds(Tile(11, 10)));
        assert!(!village_map.is_out_of_bounds(Tile(5, 5)));
    }

    #[test]
    fn test_village_map_terrain() {
        let size = TileDim(10, 10);
        let mut village_map = VillageMap::new(size);
        let tile = Tile(1, 1);
        village_map.set_terrain(tile, Terrain::Water);
        assert_eq!(village_map.get_terrain(tile), Some(Terrain::Water));
    }

    #[test]
    fn test_tile_map_creation() {
        let size = TileDim(10, 10);
        let tile_map = TileMap::new(size);
        assert_eq!(tile_map.size(), size);
        assert!(tile_map.map.is_empty());
    }

    #[test]
    fn test_tile_map_bounds() {
        let size = TileDim(10, 10);
        let tile_map = TileMap::new(size);
        let bounds = tile_map.bounds();
        assert_eq!(bounds.min(), Tile::ZERO);
        assert_eq!(bounds.max(), Tile(9, 9));
    }

    #[test]
    fn test_tile_map_is_occupied() {
        let size = TileDim(10, 10);
        let mut tile_map = TileMap::new(size);
        let tile = Tile(1, 1);
        let entity = Entity::from_raw(1);
        tile_map.set(tile, entity);
        assert!(tile_map.is_occupied(tile));
        assert!(!tile_map.is_occupied(Tile(2, 2)));
    }

    #[test]
    fn test_tile_map_get_set_remove() {
        let size = TileDim(10, 10);
        let mut tile_map = TileMap::new(size);
        let tile = Tile(1, 1);
        let entity = Entity::from_raw(1);
        tile_map.set(tile, entity);
        assert_eq!(tile_map.get(tile), Some(entity));
        assert_eq!(tile_map.remove(tile), Some(entity));
        assert_eq!(tile_map.get(tile), None);
    }

    #[test]
    fn test_tile_map_get_neighbouring_positions_rook() {
        let size = TileDim(10, 10);
        let tile_map = TileMap::new(size);
        let tile = Tile(1, 1);
        let neighbours: Vec<Tile> = tile_map.get_neighbouring_positions_rook(tile).collect();
        assert_eq!(neighbours.len(), 4);
        let expected = vec![Tile(0, 1), Tile(1, 0), Tile(1, 2), Tile(2, 1)];
        for n in neighbours {
            assert!(expected.contains(&n));
        }
    }

    #[test]
    fn test_tile_map_get_neighbouring_positions_king() {
        let size = TileDim(10, 10);
        let tile_map = TileMap::new(size);
        let tile = Tile(1, 1);
        let neighbours: Vec<Tile> = tile_map.get_neighbouring_positions_king(tile).collect();
        assert_eq!(neighbours.len(), 8);
        let expected = vec![
            Tile(0, 1),
            Tile(1, 0),
            Tile(1, 2),
            Tile(2, 1),
            Tile(0, 0),
            Tile(2, 0),
            Tile(0, 2),
            Tile(2, 2),
        ];
        for n in neighbours {
            assert!(expected.contains(&n));
        }
    }

    #[test]
    fn test_village_map_pathfind_trivial() {
        let size = TileDim(10, 10);
        let village_map = VillageMap::new(size);
        let start = Tile(3, 3);
        let target = Tile(3, 3);
        let directions = &TileDir::ALL;

        let path = village_map.pathfind(&start, &target, directions, false);
        assert!(path.is_some());
        let (tiles, cost) = path.unwrap();
        assert_eq!(tiles.first().unwrap(), &start);
        assert_eq!(tiles.last().unwrap(), &target);
        assert_eq!(cost, tiles.len() as i32 - 1);
    }

    #[test]
    fn test_village_map_pathfind_one_step() {
        let size = TileDim(1, 2);
        let village_map = VillageMap::new(size);
        let start = Tile::ZERO;
        let target = start.step(TileDir::South);
        let directions = &TileDir::ALL;

        let path = village_map.pathfind(&start, &target, directions, false);
        assert!(path.is_some());
        let (tiles, cost) = path.unwrap();
        assert_eq!(tiles.first().unwrap(), &start);
        assert_eq!(tiles.last().unwrap(), &target);
        assert_eq!(cost, tiles.len() as i32 - 1);
    }

    #[test]
    fn test_village_map_pathfind() {
        let size = TileDim(10, 10);
        let village_map = VillageMap::new(size);
        let start = Tile(0, 0);
        let target = Tile(3, 3);
        let directions = &TileDir::ALL;

        let path = village_map.pathfind(&start, &target, directions, false);
        assert!(path.is_some());
        let (tiles, cost) = path.unwrap();
        assert_eq!(tiles.last().unwrap(), &target);
        assert_eq!(cost, tiles.len() as i32 - 1);
    }

    #[test]
    fn test_village_map_flood_single_tile() {
        let size = TileDim(1, 1);
        let village_map = VillageMap::new(size);
        let start = Tile(0, 0);
        let directions = &TileDir::ALL;

        let flooded_tiles = village_map.flood(start, 3, directions, false);
        assert!(flooded_tiles.len() == 1);
        assert!(flooded_tiles.contains(&Tile(0, 0)));
    }

    #[test]
    fn test_village_map_flood_two_meridean() {
        let size = TileDim(1, 2);
        let village_map = VillageMap::new(size);
        let start = Tile(0, 0);
        let directions = &TileDir::ALL;

        let flooded_tiles = village_map.flood(start, 3, directions, false);

        assert_eq!(flooded_tiles.len(), 2);
        assert!(flooded_tiles.contains(&Tile(0, 0)));
        assert!(flooded_tiles.contains(&Tile(0, 1)));
    }

    #[test]
    fn test_village_map_flood_two_parallel() {
        let size = TileDim(2, 1);
        let village_map = VillageMap::new(size);
        let start = Tile(0, 0);
        let directions = &TileDir::ALL;

        let flooded_tiles = village_map.flood(start, 3, directions, false);
        assert_eq!(flooded_tiles.len(), 2);
        assert!(flooded_tiles.contains(&Tile(0, 0)));
        assert!(flooded_tiles.contains(&Tile(1, 0)));
    }

    #[test]
    fn test_village_map_flood_three_x_two() {
        let size = TileDim(3, 2);
        let village_map = VillageMap::new(size);
        let start = Tile(0, 0);
        let directions = &TileDir::ALL;

        let flooded_tiles = village_map.flood(start, 3, directions, false);
        println!("flooded_tiles = {flooded_tiles:?}");
        assert_eq!(flooded_tiles.len(), 6);
    }

    #[test]
    fn test_village_map_flood() {
        let size = TileDim(10, 10);
        let village_map = VillageMap::new(size);
        let start = Tile(0, 0);
        let directions = &TileDir::ALL;

        let flooded_tiles = village_map.flood(start, 3, directions, false);
        assert!(flooded_tiles.contains(&Tile(3, 3)));
    }

    #[test]
    fn test_village_map_sort_tiles_by_distance() {
        let target = Tile(0, 0);
        let mut tiles = vec![Tile(1, 1), Tile(0, 2), Tile(2, 2), Tile(3, 3)];
        VillageMap::sort_tiles_by_distance(&mut tiles, target);
        assert_eq!(tiles, vec![Tile(1, 1), Tile(0, 2), Tile(2, 2), Tile(3, 3)]);
    }

    #[test]
    fn test_village_map_sort_tiles_by_heat() {
        let mut village_map = VillageMap::new(TileDim(10, 10));
        village_map.heat_map = (0..100).collect();
        let mut tiles = vec![Tile(1, 1), Tile(2, 2), Tile(3, 3), Tile(4, 4)];
        village_map.sort_tiles_by_heat(&mut tiles);
        assert_eq!(tiles, vec![Tile(1, 1), Tile(2, 2), Tile(3, 3), Tile(4, 4)]);
    }

    #[test]
    fn test_village_map_get_worst_tile() {
        let mut village_map = VillageMap::new(TileDim(10, 10));
        village_map.heat_map = (0..100).collect();
        let start = Tile(0, 0);
        let directions = &TileDir::ALL;
        let worst_tile = village_map.get_worst_tile(start, 3, directions, false);
        assert_eq!(worst_tile, Some(Tile(3, 3)));
    }

    #[test]
    fn test_village_map_get_best_tile_trivial() {
        let mut village_map = VillageMap::new(TileDim(1, 2));
        village_map.heat_map = vec![1, 0];
        let start = Tile(0, 0);
        let directions = &TileDir::ALL;
        let best_tile = village_map.get_best_tile(start, 3, directions, false);
        assert_eq!(best_tile, Some(Tile(0, 1)));
    }

    #[test]
    fn test_village_map_generate_heat_map() {
        let mut village_map = VillageMap::new(TileDim(3, 3));
        let entity = Entity::from_raw(1);
        village_map.actors.set(Tile(2, 2), entity);
        village_map.generate_heat_map(|_| false);
        assert_eq!(village_map.heat_map[0], 4);
        assert_eq!(village_map.heat_map[8], 0);
    }
}
