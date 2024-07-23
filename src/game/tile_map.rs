use bevy::prelude::*;
use bevy::utils::HashMap;
use bimap::BiHashMap;
use bimap::Overwritten;

/// Width of a tile.
pub const TILE_SIZE: f32 = 256.0;
/// A single right direction unit in the isometric world.
pub const RIGHT_DIR: Vec2 = Vec2::new(TILE_SIZE / 2.0, -TILE_SIZE / 4.0 - 26.0);
/// A single down direction unit in the isometric world.
pub const DOWN_DIR: Vec2 = Vec2::new(-TILE_SIZE / 2.0, -TILE_SIZE / 4.0 - 26.0);

/// Z-depth of a single layer.
pub const LAYER_DEPTH: f32 = 10.0;

/// Convert tile coordinate to world translation.
pub fn tile_coord_translation(x: f32, y: f32, layer: f32) -> Vec3 {
    let mut translation = Vec3::new(RIGHT_DIR.x, RIGHT_DIR.y, -RIGHT_DIR.y) * x;
    translation += Vec3::new(DOWN_DIR.x, DOWN_DIR.y, -DOWN_DIR.y) * y;
    translation.z += layer * LAYER_DEPTH;

    translation
}

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileMap>()
            .init_resource::<TileSet>()
            .add_systems(PreStartup, load_tiles);
    }
}

#[derive(Resource, Debug, Default)]
pub struct TileMap {
    size: UVec2,
    map: BiHashMap<UVec2, Entity>,
}

impl TileMap {
    pub fn new(size: UVec2) -> TileMap {
        assert!(UVec2::ZERO.cmplt(size).all());
        TileMap {
            size,
            map: BiHashMap::default(),
        }
    }

    pub fn get(&self, position: UVec2) -> Option<Entity> {
        self.map.get_by_left(&position).copied()
    }

    pub fn locate(&self, entity: Entity) -> Option<UVec2> {
        self.map.get_by_right(&entity).copied()
    }

    pub fn set(&mut self, position: UVec2, entity: Entity) -> Overwritten<UVec2, Entity> {
        self.map.insert(position, entity)
    }

    pub fn remove(&mut self, position: UVec2) -> Option<Entity> {
        self.map.remove_by_left(&position).map(|(_, entity)| entity)
    }

    pub fn remove_entity(&mut self, entity: Entity) -> Option<UVec2> {
        self.map
            .remove_by_right(&entity)
            .map(|(position, _)| position)
    }
}

#[derive(Resource, Default, Debug)]
pub struct TileSet(HashMap<&'static str, Handle<Image>>);

impl TileSet {
    pub fn insert(&mut self, name: &'static str, handle: Handle<Image>) -> Option<Handle<Image>> {
        self.0.insert(name, handle)
    }

    /// Get cloned image handle.
    ///
    /// # Panic
    ///
    /// For ease of use, unwrap is used to panic if value does not exists for certain key.
    pub fn get(&self, name: &str) -> Handle<Image> {
        self.0.get(name).unwrap().clone()
    }
}

fn load_tiles(asset_server: Res<AssetServer>, mut tile_set: ResMut<TileSet>) {
    tile_set.insert("block_grey", asset_server.load("tiles/block_grey.png"));
    tile_set.insert("block_blue", asset_server.load("tiles/block_blue.png"));
    tile_set.insert("block_green", asset_server.load("tiles/block_green.png"));
    tile_set.insert("block_orange", asset_server.load("tiles/block_orange.png"));
}
