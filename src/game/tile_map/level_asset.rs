use bevy::{
    asset::{io::Reader, AssetLoader, AsyncReadExt, LoadContext},
    prelude::*,
    utils::HashMap,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
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
    pub tiles: Vec<String>,
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

#[derive(Resource, Default, Debug)]
pub struct Levels(pub HashMap<&'static str, Handle<LevelAsset>>);

fn load_levels(asset_sever: Res<AssetServer>, mut levels: ResMut<Levels>) {
    levels
        .0
        .insert("debug_level", asset_sever.load("levels/debug_level.json"));
}
