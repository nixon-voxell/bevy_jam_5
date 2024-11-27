use crate::path_finding::distance_map;
use crate::path_finding::find_all;
use crate::path_finding::tiles::Tile;
use crate::path_finding::tiles::TileDir;

use super::actors::stats::Health;
use super::actors::stats::Movement;
use super::actors::EnemyActor;
use super::actors::PlayerActor;
use super::actors::Structure;
use super::level::Terrain;
use super::map::VillageMap;
use crate::path_finding::tiles::Tiled;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bimap::Overwritten;

pub trait TileRef: Sized + Copy {
    fn get<G: GameData>(self, game: &G) -> Option<(Tile, Entity)> {
        self.get_tile(game)
            .and_then(move |tile| self.get_actor(game).map(move |entity| (tile, entity)))
    }
    fn get_tile<G: GameData>(self, game: &G) -> Option<Tile>;
    fn get_actor<G: GameData>(self, game: &G) -> Option<Entity>;
    fn is_contained<G: GameData>(self, game: &G) -> bool;
}

impl TileRef for Tile {
    fn get_tile<G: GameData>(self, _game: &G) -> Option<Tile> {
        Some(self)
    }

    fn get_actor<G: GameData>(self, game: &G) -> Option<Entity> {
        game.get_actor(self)
    }

    fn is_contained<G: GameData>(self, game: &G) -> bool {
        game.contains_tile(self)
    }
}

impl TileRef for Entity {
    fn get_tile<G: GameData>(self, game: &G) -> Option<Tile> {
        game.find_actor(self)
    }

    fn get_actor<G: GameData>(self, _game: &G) -> Option<Entity> {
        Some(self)
    }

    fn is_contained<G: GameData>(self, game: &G) -> bool {
        game.find_actor(self).is_some()
    }
}

pub trait GameData {
    fn get_actor(&self, r: impl TileRef) -> Option<Entity>;
    fn find_actor(&self, entity: Entity) -> Option<Tile>;
    fn contains_tile(&self, tile: Tile) -> bool;
}

/// System param for accessing game data
#[derive(SystemParam)]
pub struct Game<'w, 's> {
    pub map: ResMut<'w, VillageMap>,
    pub health: Query<'w, 's, &'static mut Health>,
    pub movement: Query<'w, 's, &'static mut Movement>,
    pub player_actors: Query<'w, 's, Entity, With<PlayerActor>>,
    pub enemy_actors: Query<'w, 's, Entity, With<EnemyActor>>,
    pub structures: Query<'w, 's, Entity, With<Structure>>,
}

impl GameData for Game<'_, '_> {
    fn get_actor(&self, r: impl TileRef) -> Option<Entity> {
        r.get_tile(self).and_then(|tile| self.map.actors.get(tile))
    }

    fn find_actor(&self, entity: Entity) -> Option<Tile> {
        self.map.actors.locate(entity)
    }

    fn contains_tile(&self, tile: Tile) -> bool {
        self.map.contains_tile(tile)
    }
}

impl Game<'_, '_> {
    pub fn terrain(&self, r: impl TileRef) -> Option<Terrain> {
        r.get_tile(self).and_then(|tile| self.map.get_terrain(tile))
    }

    pub fn tiles(&self) -> impl Iterator<Item = Tile> {
        self.map.bounds().into_iter()
    }

    pub fn actors(&self) -> impl Iterator<Item = (Tile, Entity)> + '_ {
        self.map.actors.iter()
    }

    pub fn is_occupied(&self, tile: Tile) -> bool {
        self.map.actors.is_occupied(tile)
    }

    pub fn contains(&self, r: impl TileRef) -> bool {
        r.is_contained(self)
    }

    pub fn get(&self, r: impl TileRef) -> Option<(Tile, Entity)> {
        r.get(self)
    }

    pub fn perimeter(&self) -> impl Iterator<Item = Tile> + '_ {
        self.map.find_perimeter(&TileDir::EDGES)
    }

    pub fn edge_adjacent(&self, tile: Tile) -> impl Iterator<Item = Tile> + '_ {
        tile.edge_adjacent()
            .into_iter()
            .filter(|tile| self.contains(*tile))
    }

    pub fn remove(&mut self, r: impl TileRef) -> Option<(Tile, Entity)> {
        r.get_tile(self)
            .and_then(move |tile| (self.map.actors.remove(tile).map(|entity| (tile, entity))))
    }

    pub fn structures(&self) -> impl Iterator<Item = (Tile, Entity)> + '_ {
        self.structures.iter().filter_map(|entity| self.get(entity))
    }

    pub fn insert(&mut self, tile: Tile, entity: Entity) -> InsertedActor {
        assert!(
            self.contains(tile),
            "{tile:?} outside of map bounds {:?}",
            self.map.bounds()
        );
        match self.map.actors.set(tile, entity) {
            Overwritten::Left(_, entity) => InsertedActor::ReplacedActor(entity),
            Overwritten::Right(tile, _) => InsertedActor::MovedFrom(tile),
            _ => InsertedActor::Placed,
        }
    }

    pub fn all_structures_reachable(&self) -> bool {
        let Some(start) = self.perimeter().find(|tile| {
            self.terrain(*tile)
                .map(|terrain| terrain.is_walkable())
                .unwrap_or(false)
        }) else {
            return false;
        };

        let structures: HashSet<Tile> = self.structures().map(|(tile, _)| tile).collect();
        let navigator = |tile| {
            let structures = &structures;
            self.edge_adjacent(tile)
                .filter(move |_| !structures.contains(&tile))
        };
        find_all(start, navigator).is_superset(&structures)
    }

    /// A tile can be built on if
    /// - it is walkable
    /// - it is not on the map perimeter
    /// - from the tile there exists a walkable path to every perimeter tile on the map
    /// - it is within two tiles distance (by edge steps) of another structure
    ///     (losing all buildings is defeat)
    pub fn find_tiles_that_can_be_built_on(&self) -> HashSet<Tile> {
        let structures: HashSet<Tile> = self.structures().map(|(tile, _)| tile).collect();

        let distance_map = distance_map(structures.iter().copied(), |t| {
            t.edge_adjacent()
                .into_iter()
                .filter(|tile| self.contains(*tile))
        });

        let perimeter: HashSet<Tile> = self.perimeter().collect();

        let candidate_tiles: HashSet<Tile> = self
            .tiles()
            .filter(|tile| {
                // must be walkable
                self.terrain(*tile)
                    .filter(|terrain| terrain.is_walkable())
                    .is_some()
                // and not already built on
                && !structures.contains(tile)
                // within two tiles distance by edges
                && distance_map.get(tile).map(|distance| *distance < 3).unwrap_or(false)
                // not on perimeter
                && !perimeter.contains(tile)
            })
            .collect();

        // pick any structure, shouldn't matter which
        let Some(start) = structures.iter().next().copied() else {
            // no structures, empty return
            return HashSet::default();
        };

        // navigator only enters candidates
        let navigator = |tile: Tile| {
            tile.edge_adjacent()
                .into_iter()
                .filter(|adj_tile| candidate_tiles.contains(adj_tile))
        };

        let mut reachable = find_all(start, navigator);
        reachable.remove(&start);
        reachable
    }

    pub fn iter_terrain(&self) -> impl Iterator<Item = (Tile, Terrain)> + '_ {
        self.map.iter_terrain()
    }

    pub fn deployment_zone(&self) -> &HashSet<Tile> {
        &self.map.deployment_zone
    }
}

/// System param for accessing game data
#[derive(SystemParam)]
pub struct ReadGame<'w, 's> {
    pub map: Option<Res<'w, VillageMap>>,
    health: Query<'w, 's, &'static Health>,
    movement: Query<'w, 's, &'static Movement>,
    player_actors: Query<'w, 's, Entity, With<PlayerActor>>,
    enemy_actors: Query<'w, 's, Entity, With<EnemyActor>>,
    structures: Query<'w, 's, Entity, With<Structure>>,
}

impl GameData for ReadGame<'_, '_> {
    fn get_actor(&self, r: impl TileRef) -> Option<Entity> {
        r.get_tile(self)
            .and_then(|tile| self.map.as_ref().unwrap().actors.get(tile))
    }

    fn find_actor(&self, entity: Entity) -> Option<Tile> {
        self.map.as_ref().unwrap().actors.locate(entity)
    }

    fn contains_tile(&self, tile: Tile) -> bool {
        self.map.as_ref().unwrap().contains_tile(tile)
    }
}

impl ReadGame<'_, '_> {
    pub fn map(&self) -> &VillageMap {
        self.map.as_ref().unwrap()
    }

    pub fn terrain(&self, r: impl TileRef) -> Option<Terrain> {
        r.get_tile(self)
            .and_then(|tile| self.map().get_terrain(tile))
    }

    pub fn tiles(&self) -> impl Iterator<Item = Tile> {
        self.map().bounds().into_iter()
    }

    pub fn actors(&self) -> impl Iterator<Item = (Tile, Entity)> + '_ {
        self.map().actors.iter()
    }

    pub fn is_occupied(&self, tile: Tile) -> bool {
        self.map().actors.is_occupied(tile)
    }

    pub fn contains(&self, r: impl TileRef) -> bool {
        r.is_contained(self)
    }

    pub fn get(&self, r: impl TileRef) -> Option<(Tile, Entity)> {
        r.get(self)
    }

    pub fn perimeter(&self) -> impl Iterator<Item = Tile> + '_ {
        self.map().find_perimeter(&TileDir::EDGES)
    }

    pub fn edge_adjacent(&self, tile: Tile) -> impl Iterator<Item = Tile> + '_ {
        tile.edge_adjacent()
            .into_iter()
            .filter(|tile| self.contains(*tile))
    }

    pub fn structures(&self) -> impl Iterator<Item = (Tile, Entity)> + '_ {
        self.structures.iter().filter_map(|entity| self.get(entity))
    }

    pub fn all_structures_reachable(&self) -> bool {
        let Some(start) = self.perimeter().find(|tile| {
            self.terrain(*tile)
                .map(|terrain| terrain.is_walkable())
                .unwrap_or(false)
        }) else {
            return false;
        };

        let structures: HashSet<Tile> = self.structures().map(|(tile, _)| tile).collect();
        let navigator = |tile| {
            let structures = &structures;
            self.edge_adjacent(tile)
                .filter(move |_| !structures.contains(&tile))
        };
        find_all(start, navigator).is_superset(&structures)
    }

    /// A tile can be built on if
    /// - it is walkable
    /// - it is not on the map perimeter
    /// - from the tile there exists a walkable path to every perimeter tile on the map
    /// - it is within two tiles distance (by edge steps) of another structure
    ///     (losing all buildings is defeat)
    pub fn find_tiles_that_can_be_built_on(&self) -> HashSet<Tile> {
        let structures: HashSet<Tile> = self.structures().map(|(tile, _)| tile).collect();

        let distance_map = distance_map(structures.iter().copied(), |t| {
            t.edge_adjacent()
                .into_iter()
                .filter(|tile| self.contains(*tile))
        });

        let perimeter: HashSet<Tile> = self.perimeter().collect();

        let candidate_tiles: HashSet<Tile> = self
            .tiles()
            .filter(|tile| {
                // must be walkable
                self.terrain(*tile)
                    .filter(|terrain| terrain.is_walkable())
                    .is_some()
                // and not already built on
                && !structures.contains(tile)
                // within two tiles distance by edges
                && distance_map.get(tile).map(|distance| *distance < 3).unwrap_or(false)
                // not on perimeter
                && !perimeter.contains(tile)
            })
            .collect();

        // pick any structure, shouldn't matter which
        let Some(start) = structures.iter().next().copied() else {
            // no structures, empty return
            return HashSet::default();
        };

        // navigator only enters candidates
        let navigator = |tile: Tile| {
            tile.edge_adjacent()
                .into_iter()
                .filter(|adj_tile| candidate_tiles.contains(adj_tile))
        };

        let mut reachable = find_all(start, navigator);
        reachable.remove(&start);
        reachable
    }

    pub fn iter_terrain(&self) -> impl Iterator<Item = (Tile, Terrain)> + '_ {
        self.map().iter_terrain()
    }

    pub fn deployment_zone(&self) -> &HashSet<Tile> {
        &self.map().deployment_zone
    }
}

#[derive(Debug, Copy, Clone)]
pub enum InsertedActor {
    /// The tile was empty and the actor did not already exist in the world map.
    Placed,
    /// The actor already existed in the world map. Returns its previous tile.
    #[allow(unused)]
    MovedFrom(Tile),
    /// The actor replaced an existing tile. Returns the replaced actor.
    #[allow(unused)]
    ReplacedActor(Entity),
}
