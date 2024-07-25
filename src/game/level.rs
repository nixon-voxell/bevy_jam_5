//! Spawn the main level by triggering other observers.

use bevy::{math::vec2, prelude::*};

use crate::{screen::Screen, VillageCamera};

use self::level_asset::{LevelAsset, LevelAssetPlugin, Levels};

use super::{
    map::VillageMap,
    tile_set::{tile_coord_translation, PickableTile, TileSet, TILE_ANCHOR, TILE_HALF_HEIGHT},
};

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
    mut village_camera_query: Query<&mut Transform, With<VillageCamera>>,
) {
    // Choose a random level
    let level_index = rand::random::<usize>() % levels.0.len();
    let level = &mut levels.0[level_index];

    let Some(level_asset) = level_assets.get(&level.handle) else {
        error!("Unable to load level: {}", level.name);
        return;
    };

    

    let mut village_map = VillageMap::new(UVec2::splat(level_asset.size as u32));

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
            let ground_translation =  tile_coord_translation(xf, yf, 0.0);
            let edge_translation = tile_coord_translation(xf, yf, 1.0);
            let object_translation = tile_coord_translation(xf, yf, 2.0);

            let (xi, yi) = (x as i32, y as i32);
            village_map.ground.set(
                IVec2::new(xi, yi),
                commands
                    .spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                anchor: TILE_ANCHOR,
                                ..Default::default()
                            },
                            texture: tile_set.get(ground_tile_name),
                            transform: Transform::from_translation(ground_translation),
                            ..default()
                        },
                        PickableTile,
                        StateScoped(Screen::Playing),
                    ))
                    .id(),
            );

            for s in [Vec2::ONE, vec2(1., -1.), -Vec2::ONE, vec2(-1., 1.)] {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            anchor: TILE_ANCHOR,
                            ..Default::default()
                        },
                        texture: tile_set.get("edge"),
                        transform: Transform {
                            translation: edge_translation,
                            scale: s.extend(1.),
                            ..Default::default()
                        },
                        visibility: Visibility::Hidden,
                        ..default()
                    },
                    StateScoped(Screen::Playing),
                ));
            }

            if object_tile_name != "empty" {
                village_map.object.set(
                    IVec2::new(xi, yi),
                    commands
                        .spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    anchor: TILE_ANCHOR,
                                    ..Default::default()
                                },
                                texture: tile_set.get(object_tile_name),
                                transform: Transform::from_translation(object_translation),
                                ..default()
                            },
                            PickableTile,
                            StateScoped(Screen::Playing),
                        ))
                        .id(),
                );
            }
        }
    }
    // level_asset.create_edges(&mut commands, &tile_set);

    commands.insert_resource(village_map);
}
