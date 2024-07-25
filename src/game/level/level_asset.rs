use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext, LoadState},
    math::vec2,
    prelude::*,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    game::tile_map::{tile_coord_translation, PickableTile, TileSet, TILE_HALF_HEIGHT},
    screen::Screen,
};

pub struct LevelAssetPlugin;

impl Plugin for LevelAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LevelAsset>()
            .init_asset_loader::<LevelAssetLoader>()
            .init_resource::<Levels>()
            .add_systems(PreStartup, load_levels);
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
                    .spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                anchor: bevy::sprite::Anchor::Custom(vec2(0., 0.5 - 293. / 512.)),
                                ..Default::default()
                            },
                            texture: tile_set.get(name),
                            transform: Transform::from_translation(translation),
                            ..default()
                        },
                        PickableTile,
                    ))
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

    pub fn create_edges(
        &self,
        commands: &mut Commands,
        tile_set: &TileSet,
    ) {
       
            let start_translation = Vec3::new(
                0.0,
                TILE_HALF_HEIGHT * self.size as f32 - TILE_HALF_HEIGHT,
                0.0,
            );
            for i in 0..self.tiles[0].len() {
                
                let x = (i % self.size) as f32;
                let y = (i / self.size) as f32;
                let translation =
                    start_translation + tile_coord_translation(x, y, 1.);
                
                for s in [
                    Vec2::ONE,
                    vec2(1., -1.),
                    -Vec2::ONE,
                    vec2(-1., 1.)
                ] {

                    commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            anchor: bevy::sprite::Anchor::Custom(vec2(0., 0.5 - 293. / 512.)),
                            ..Default::default()
                        },
                        texture: tile_set.get("edge"),
                        transform: Transform { translation, scale: s.extend(1.), ..Default::default() },
                        visibility: Visibility::Hidden,
                        ..default()                    
                    }
                    

                );
                }
            }
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
    pub name: String,
    pub handle: Handle<LevelAsset>,
}

impl LevelLoad {
    pub fn new(name: String, handle: Handle<LevelAsset>) -> Self {
        Self { name, handle }
    }
}

#[derive(Resource, Default, Debug)]
pub struct Levels(pub Vec<LevelLoad>);

/// Load levels from json file.
fn load_levels(asset_sever: Res<AssetServer>, mut levels: ResMut<Levels>) {
    info!("Loading levels from json...");

    const LEVELS: &[&str] = &["debug_level"];

    for &level in LEVELS {
        levels.0.push(LevelLoad::new(
            String::from(level),
            asset_sever.load(format!("levels/{}.json", level)),
        ));
    }
}
