use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext, LoadState},
    prelude::*,
    utils::HashMap,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    game::tile_map::{tile_coord_translation, TileSet, TILE_HALF_HEIGHT},
    screen::Screen,
};

pub struct LevelAssetPlugin;

impl Plugin for LevelAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LevelAsset>()
            .init_asset_loader::<LevelAssetLoader>()
            .init_resource::<Levels>()
            .add_systems(PreStartup, load_levels)
            .add_systems(Update, prespawn_levels)
            .add_systems(OnExit(Screen::Playing), reset_levels);
    }
}

#[derive(Asset, TypePath, Serialize, Deserialize)]
pub struct LevelAsset {
    pub name: String,
    pub size: usize,
    pub tiles: [Vec<String>; 2],
}

impl LevelAsset {
    fn create_tile_entities(
        &self,
        commands: &mut Commands,
        tile_set: &TileSet,
        layer: usize,
        translation_layer: f32,
    ) -> Vec<Entity> {
        let start_translation = Vec3::new(
            0.0,
            TILE_HALF_HEIGHT * self.size as f32 - TILE_HALF_HEIGHT,
            0.0,
        );
        self.tiles[layer]
            .iter()
            .enumerate()
            .filter(|(_, name)| *name != "empty")
            .map(|(i, name)| {
                let x = (i % self.size) as f32;
                let y = (i / self.size) as f32;
                let translation =
                    start_translation + tile_coord_translation(x, y, translation_layer);

                commands
                    .spawn(SpriteBundle {
                        texture: tile_set.get(name),
                        transform: Transform::from_translation(translation),
                        ..default()
                    })
                    .id()
            })
            .collect()
    }

    pub fn create_ground_entities(
        &self,
        commands: &mut Commands,
        tile_set: &TileSet,
    ) -> Vec<Entity> {
        self.create_tile_entities(commands, tile_set, 0, 0.0)
    }

    pub fn create_object_entities(
        &self,
        commands: &mut Commands,
        tile_set: &TileSet,
    ) -> Vec<Entity> {
        // 1 layer higher because we want a middle layer to place
        // interaction tiles (on hover, on click, etc.).
        self.create_tile_entities(commands, tile_set, 1, 2.0)
    }
}

#[derive(Default)]
pub struct LevelAssetLoader;

impl AssetLoader for LevelAssetLoader {
    type Asset = LevelAsset;
    type Settings = ();
    type Error = LevelAssetLoaderError;

    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let tile_map_asset = serde_json::from_slice::<LevelAsset>(&bytes)?;

        Ok(tile_map_asset)
    }

    fn extensions(&self) -> &[&str] {
        &[".json"]
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LevelAssetLoaderError {
    #[error("Could not load json file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not deserialize using serde: {0}")]
    Serde(#[from] serde_json::Error),
}

/// Stores [`LevelAsset`] as well as their parent [`Entity`] if it is already spawned.
#[derive(Debug)]
pub struct LevelLoad {
    pub handle: Handle<LevelAsset>,
    pub parent: Entity,
    pub state: LoadState,
}

impl LevelLoad {
    pub fn new(handle: Handle<LevelAsset>, parent: Entity) -> Self {
        Self {
            handle,
            parent,
            state: LoadState::NotLoaded,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.state == LoadState::Loaded
    }
}

#[derive(Resource, Default, Debug)]
pub struct Levels(pub HashMap<&'static str, LevelLoad>);

#[derive(Component)]
pub struct LevelMarker;

/// Load levels from json file.
fn load_levels(mut commands: Commands, asset_sever: Res<AssetServer>, mut levels: ResMut<Levels>) {
    info!("Loading levels from json...");
    levels.0.insert(
        "debug_level",
        LevelLoad::new(
            asset_sever.load("levels/debug_level.json"),
            // Hidden by default, set to visible to "load" map
            commands
                .spawn((
                    SpatialBundle {
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    LevelMarker,
                ))
                .id(),
        ),
    );
}

/// Prespawn levels so that we can easily load/unload them by changing the parent's visibility.
fn prespawn_levels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut levels: ResMut<Levels>,
    level_assets: Res<Assets<LevelAsset>>,
    tile_set: Res<TileSet>,
) {
    for (name, level) in levels.0.iter_mut() {
        if level.is_loaded() {
            continue;
        }

        let Some(load_state) = asset_server.get_load_state(&level.handle) else {
            warn!("No load state for level: {:?}..", level.handle);
            return;
        };

        if let LoadState::Loaded = load_state {
            let debug_level = level_assets.get(&level.handle).unwrap();
            info!("Loading level: {name}");

            let ground_tiles = debug_level.create_ground_entities(&mut commands, &tile_set);
            let object_tiles = debug_level.create_object_entities(&mut commands, &tile_set);

            commands.entity(level.parent).despawn_descendants();
            commands.entity(level.parent).push_children(&ground_tiles);
            commands.entity(level.parent).push_children(&object_tiles);
        }
        level.state = load_state.clone();
    }
}

/// Reset level load state so that [`prespawn_levels`] can kick in.
fn reset_levels(mut levels: ResMut<Levels>) {
    for level in levels.0.values_mut() {
        level.state = LoadState::Loading;
    }
}
