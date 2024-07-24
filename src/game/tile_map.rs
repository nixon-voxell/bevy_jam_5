use bevy::prelude::*;
use bevy::utils::HashMap;
use bimap::BiHashMap;
use bimap::Overwritten;

/// Width of a tile.
pub const TILE_WIDTH: f32 = 256.0;
/// Half height of a tile surface.
pub const TILE_HALF_HEIGHT: f32 = TILE_WIDTH / 4.0 + 26.0;
/// A single right direction unit in the isometric world.
pub const RIGHT_DIR: Vec2 = Vec2::new(TILE_WIDTH / 2.0, -TILE_HALF_HEIGHT);
/// A single down direction unit in the isometric world.
pub const DOWN_DIR: Vec2 = Vec2::new(-TILE_WIDTH / 2.0, -TILE_HALF_HEIGHT);

/// Z-depth of a single layer.
pub const LAYER_DEPTH: f32 = 10.0;

/// Convert tile coordinate to world translation.
pub fn tile_coord_translation(x: f32, y: f32, layer: f32) -> Vec3 {
    let mut translation = RIGHT_DIR.xyy() * x;
    translation += DOWN_DIR.xyy() * y;
    translation.z = translation.z * -0.001 + layer * LAYER_DEPTH;

    translation
}



pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileMap>()
            .init_resource::<TileSet>()
            .init_resource::<PickedTile>()
            .add_systems(PreStartup, load_tiles);
    }
}

#[derive(Resource, Debug, Default)]
pub struct TileMap {
    size: IVec2,
    map: BiHashMap<IVec2, Entity>,
}

/// movement directions on tilemap
pub const NORTH: IVec2 = IVec2::Y;
pub const EAST: IVec2 = IVec2::X;
pub const SOUTH: IVec2 = IVec2 { y: -1, x: 0 };
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

impl TileMap {
    pub fn new(size: IVec2) -> TileMap {
        assert!(IVec2::ZERO.cmplt(size).all());
        TileMap {
            size,
            map: BiHashMap::default(),
        }
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

    pub fn get_neighbouring_positions_rook<'a>(
        &'a self,
        position: IVec2,
    ) -> impl Iterator<Item = IVec2> + 'a {
        ROOK_MOVES
            .iter()
            .copied()
            .map(move |translation| position + translation)
            .filter(|target| self.bounds().contains(*target))
    }

    pub fn get_neighbouring_positions_king<'a>(
        &'a self,
        position: IVec2,
    ) -> impl Iterator<Item = IVec2> + 'a {
        KING_MOVES
            .iter()
            .copied()
            .map(move |translation| position + translation)
            .filter(|target| self.bounds().contains(*target))
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
    tile_set.insert("grassblock", asset_server.load("tiles/grassblock.png"));
    tile_set.insert("werewolf", asset_server.load("tiles/werewolf.png"));
}


#[derive(Resource, Default, Debug)]
pub struct PickedTile(pub Option<IVec2>);