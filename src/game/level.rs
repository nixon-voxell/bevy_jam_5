//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use crate::screen::Screen;

use self::level_asset::{LevelAsset, LevelAssetPlugin, Levels};

use super::tile_map::TileSet;

pub mod level_asset;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LevelAssetPlugin)
            .add_systems(OnEnter(Screen::Playing), load_level);
    }
}

fn load_level(
    mut commands: Commands,
    mut levels: ResMut<Levels>,
    level_assets: Res<Assets<LevelAsset>>,
    tile_set: Res<TileSet>,
) {
    let level_index = rand::random::<usize>() % levels.0.len();
    let level = &mut levels.0[level_index];

    let Some(level_asset) = level_assets.get(&level.handle) else {
        error!("Unable to load level: {}", level.name);
        return;
    };

    let ground_tiles = level_asset.create_ground_entities(&mut commands, &tile_set);
    let object_tiles = level_asset.create_object_entities(&mut commands, &tile_set);

    commands
        .spawn((StateScoped(Screen::Playing), SpatialBundle::default()))
        .push_children(&ground_tiles)
        .push_children(&object_tiles);
}
