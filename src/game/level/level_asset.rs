use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct LevelAssetPlugin;

impl Plugin for LevelAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LevelAsset>()
            .init_asset_loader::<LevelAssetLoader>()
            .init_resource::<Levels>()
            .add_systems(PreStartup, load_levels);
    }
}

/// Load levels from json file.
fn load_levels(asset_sever: Res<AssetServer>, mut levels: ResMut<Levels>) {
    const LEVELS: &[&str] = &["debug_level"];

    for &level in LEVELS {
        info!("Loading level: {}", level);

        levels.0.push(LevelLoad::new(
            String::from(level),
            asset_sever.load(format!("levels/{}.json", level)),
        ));
    }
}

#[derive(Asset, TypePath, Serialize, Deserialize)]
pub struct LevelAsset {
    pub name: String,
    pub size: usize,
    pub tiles: [Vec<String>; 2],
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
