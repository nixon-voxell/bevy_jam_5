//! Spawn the main level by triggering other observers.

use bevy::color::palettes::css::YELLOW;
use bevy::prelude::*;

use crate::game::unit::spawn::SpawnAnimation;
use crate::game::unit::StructureBundle;
use crate::path_finding::tiles::{Tile, TileDim};
use crate::{game::components::GroundTileLayer, screen::Screen, VillageCamera};

use super::unit::EnemyUnit;
use super::{picking::PickableTile, selection::SelectionMap};

use self::level_asset::{LevelAsset, LevelAssetPlugin, Levels};

use super::{
    map::VillageMap,
    tile_set::{tile_coord_translation, TileSet, TILE_ANCHOR, TILE_HALF_HEIGHT},
};

pub mod level_asset;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LevelAssetPlugin)
            .add_systems(OnEnter(Screen::Playing), load_level);
    }
}

/// Marker component for a sprite that shows a line around the edges of a tile.
#[derive(Component)]
pub struct TileBorder;

/// Marker component for a sprite that shows a thick line around the edges of a tile.
#[derive(Component)]
pub struct TileThickBorder;

pub fn load_level(
    mut commands: Commands,
    mut village_camera_query: Query<&mut Transform, With<VillageCamera>>,
    enemies_query: Query<(), With<EnemyUnit>>,
    mut levels: ResMut<Levels>,
    level_assets: Res<Assets<LevelAsset>>,
    tile_set: Res<TileSet>,
    asset_server: Res<AssetServer>,
) {
    // Choose a random level
    let level_index = rand::random::<usize>() % levels.0.len();
    let level = &mut levels.0[level_index];

    let Some(level_asset) = level_assets.get(&level.handle) else {
        error!("Unable to load level: {}", level.name);
        return;
    };

    let mut selection_map = SelectionMap::default();
    let mut village_map = VillageMap::new(TileDim::splat(level_asset.size as i32));

    let camera_translation = Vec3::new(
        0.0,
        -TILE_HALF_HEIGHT * level_asset.size as f32 + TILE_HALF_HEIGHT,
        0.0,
    );

    for mut transform in village_camera_query.iter_mut() {
        transform.translation = camera_translation;
    }

    for y in 0..level_asset.size {
        for x in 0..level_asset.size {
            let index = x + y * level_asset.size;

            let ground_tile_name = &level_asset.tiles[0][index];
            let object_tile_name = &level_asset.tiles[1][index];

            let (xf, yf) = (x as f32, y as f32);

            let edge_translation = tile_coord_translation(xf, yf, 1.0);
            let object_translation = tile_coord_translation(xf, yf, 2.0);

            let (xi, yi) = (x as i32, y as i32);

            let terrain = match ground_tile_name.as_str() {
                "grassblock" => Terrain::Grass,
                "gravelblock" => Terrain::Gravel,
                "waterblock" => Terrain::Water,
                _ => {
                    warn!("Spawning unknown tile: {}", ground_tile_name);
                    Terrain::Gravel
                }
            };

            village_map.set_terrain(Tile(xi, yi), terrain);

            // Border
            let id = commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            anchor: TILE_ANCHOR,
                            color: YELLOW.into(),
                            ..Default::default()
                        },
                        texture: tile_set.get("border"),
                        transform: Transform {
                            translation: edge_translation + Vec3::Z * 2.0,
                            ..Default::default()
                        },
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    StateScoped(Screen::Playing),
                    Tile(xi, yi),
                    TileBorder,
                ))
                .id();
            selection_map.borders.insert(Tile(xi, yi), id);
            // Thick border
            let id = commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            anchor: TILE_ANCHOR,
                            color: YELLOW.into(),
                            ..Default::default()
                        },
                        texture: tile_set.get("border_thick"),
                        transform: Transform {
                            translation: edge_translation + Vec3::Z,
                            ..Default::default()
                        },
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    StateScoped(Screen::Playing),
                    Tile(xi, yi),
                    TileThickBorder,
                ))
                .id();
            selection_map.thick_borders.insert(Tile(xi, yi), id);

            if object_tile_name != "empty" {
                let object_entity = commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            anchor: TILE_ANCHOR,
                            ..Default::default()
                        },
                        texture: tile_set.get(object_tile_name),
                        ..default()
                    },
                    PickableTile,
                    StateScoped(Screen::Playing),
                    // IMPORTANT: There must only be structure in the map asset
                    StructureBundle::default(),
                    SpawnAnimation::new(object_translation),
                ));

                village_map.object.set(Tile(xi, yi), object_entity.id());
            }
        }
    }

    village_map.generate_heat_map(|e| enemies_query.contains(e));
    commands.insert_resource(village_map);
    commands.insert_resource(selection_map)
}

#[derive(Component, Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Terrain {
    #[default]
    /// Tile is grassland.
    Grass,
    /// Tile is gravel.
    Gravel,
    /// Tile is water (land units cannot be on top of this tile).
    Water,
}
