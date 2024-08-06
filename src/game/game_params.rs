use crate::path_finding::tiles::Tile;

use super::actors::stats::Health;
use super::actors::stats::Movement;
use super::actors::EnemyActor;
use super::actors::PlayerActor;
use super::level::Terrain;
use super::map::VillageMap;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

pub trait TileRef: Sized + Copy {
    fn get(self, game: &Game) -> Option<(Tile, Entity)> {
        self.get_tile(game)
            .and_then(move |tile| self.get_actor(game).map(move |entity| (tile, entity)))
    }
    fn get_tile(self, game: &Game) -> Option<Tile>;
    fn get_actor(self, game: &Game) -> Option<Entity>;
    fn is_contained(self, game: &Game) -> bool;
}

impl TileRef for Tile {
    fn get_tile(self, _game: &Game) -> Option<Tile> {
        Some(self)
    }

    fn get_actor(self, game: &Game) -> Option<Entity> {
        game.get_actor(self)
    }

    fn is_contained(self, game: &Game) -> bool {
        game.map.contains_tile(self)
    }
}

impl TileRef for Entity {
    fn get_tile(self, game: &Game) -> Option<Tile> {
        game.find_actor(self)
    }

    fn get_actor(self, _game: &Game) -> Option<Entity> {
        Some(self)
    }

    fn is_contained(self, game: &Game) -> bool {
        game.find_actor(self).is_some()
    }
}

/// System param for accessing game data
#[derive(SystemParam)]
pub struct Game<'w, 's> {
    map: ResMut<'w, VillageMap>,
    health: Query<'w, 's, &'static mut Health>,
    movement: Query<'w, 's, &'static mut Movement>,
    player_actors: Query<'w, 's, Entity, With<PlayerActor>>,
    enemy_actors: Query<'w, 's, Entity, With<EnemyActor>>,
}

impl Game<'_, '_> {
    pub fn terrain(&self, tile: Tile) -> Option<Terrain> {
        self.map.get_terrain(tile)
    }

    pub fn get_actor(&self, tile: Tile) -> Option<Entity> {
        self.map.actors.get(tile)
    }

    pub fn find_actor(&self, entity: Entity) -> Option<Tile> {
        self.map.actors.locate(entity)
    }

    pub fn tiles(&self) -> impl Iterator<Item = Tile> {
        self.map.bounds().into_iter()
    }

    pub fn actors(&self) -> impl Iterator<Item = (Tile, Entity)> + '_ {
        self.map.actors.iter()
    }

    pub fn contains(&self, r: impl TileRef) -> bool {
        r.is_contained(self)
    }

    pub fn get(&self, r: impl TileRef) -> Option<(Tile, Entity)> {
        r.get(self)
    }
}
