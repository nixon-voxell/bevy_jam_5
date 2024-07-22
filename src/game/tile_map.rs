use bevy::prelude::*;
use bevy::utils::HashMap;
use bimap::BiHashMap;
use bimap::Overwritten;

pub const TILE_SIZE: f32 = 256.0;
pub const RIGHT_DIR: Vec2 = Vec2::new(TILE_SIZE / 2.0, -TILE_SIZE / 4.0);
pub const DOWN_DIR: Vec2 = Vec2::new(-TILE_SIZE / 2.0, -TILE_SIZE / 4.0);

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileMap>()
            .init_resource::<TileSet>()
            .add_systems(Startup, load_tiles)
            .add_systems(PostStartup, load_level);
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
pub struct TileSet(HashMap<TileType, Handle<Image>>);

impl TileSet {
    pub fn insert(&mut self, ty: TileType, handle: Handle<Image>) -> Option<Handle<Image>> {
        self.0.insert(ty, handle)
    }

    /// Get cloned image handle.
    ///
    /// # Panic
    ///
    /// For ease of use, unwrap is used to panic if value does not exists for certain key.
    pub fn get(&self, ty: TileType) -> Handle<Image> {
        self.0.get(&ty).unwrap().clone()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TileType {
    Block,
}

#[derive(Component, Clone, Copy)]
pub struct Tile;

fn load_tiles(asset_server: Res<AssetServer>, mut tile_set: ResMut<TileSet>) {
    tile_set.insert(TileType::Block, asset_server.load("tiles/block.png"));
}

fn load_level(mut commands: Commands, tile_set: Res<TileSet>) {
    let tile_size = 10;
    let start_translation = Vec3::new(0.0, 600.0, 0.0);

    for y in 0..tile_size {
        for x in 0..tile_size {
            let mut translation = start_translation;
            translation += RIGHT_DIR.xyy() * x as f32;
            translation += DOWN_DIR.xyy() * y as f32;
            translation.z *= -0.001;

            commands.spawn(SpriteBundle {
                texture: tile_set.get(TileType::Block),
                transform: Transform::from_translation(translation),
                ..default()
            });
        }
    }
}
