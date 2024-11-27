use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::utils::HashMap;

use super::level::Terrain;

/// Width of a tile.
pub const TILE_WIDTH: f32 = 256.0;
/// Half height of a tile surface.
pub const TILE_HALF_HEIGHT: f32 = TILE_WIDTH / 4.0 + 26.0;
/// A single right direction unit in the isometric world.
pub const RIGHT_DIR: Vec2 = Vec2::new(-TILE_WIDTH / 2.0, -TILE_HALF_HEIGHT);
/// A single down direction unit in the isometric world.
pub const DOWN_DIR: Vec2 = Vec2::new(TILE_WIDTH / 2.0, -TILE_HALF_HEIGHT);

/// Z-depth of a single layer.
pub const LAYER_DEPTH: f32 = 10.0;

pub const TILE_ANCHOR_VEC: Vec2 = Vec2 {
    x: 0.,
    y: 0.5 - 293. / 512.,
};

pub const TILE_ANCHOR: Anchor = Anchor::Custom(TILE_ANCHOR_VEC);

pub struct TileSetPlugin;

impl Plugin for TileSetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileSet>()
            .add_systems(PreStartup, load_tiles);
    }
}

fn load_tiles(asset_server: Res<AssetServer>, mut tile_set: ResMut<TileSet>) {
    const TILES: &[&str] = &[
        "edge",
        "grassblock",
        "gravelblock",
        "waterblock",
        "house1",
        "blacksmith",
        "human",
        "fighter",
        "warrior",
        "werewolf",
        "spartan",
        "viking",
        "slime",
        "bat",
        "border_thick",
        "border",
        "tower",
        "tavern",
        "ne_corner",
        "se_corner",
        "block_blue",
        "block_grey",
        "block_orange",
    ];

    for &tile in TILES {
        info!("Loading tile: {}", tile);
        tile_set.insert(tile, asset_server.load(format!("tiles/{}.png", tile)));
    }
}

/// Convert tile coordinate to world translation.
pub fn tile_coord_translation(x: f32, y: f32, layer: f32) -> Vec3 {
    let mut translation = RIGHT_DIR.xyy() * x;
    translation += DOWN_DIR.xyy() * y;
    let z_rank = x * 15. + y * 10.;
    translation.z = translation.z * -0.001 + layer * LAYER_DEPTH + z_rank;

    translation
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
        match self.0.get(name) {
            Some(handle) => handle.clone(),
            None => panic!("Unable to get tile: {name}"),
        }
    }

    pub fn get_terrain(&self, terrain: Terrain) -> Handle<Image> {
        let image_name = match terrain {
            Terrain::Grass => "grassblock",
            Terrain::Gravel => "gravelblock",
            Terrain::Water => "waterblock",
        };
        self.get(image_name)
    }
}
