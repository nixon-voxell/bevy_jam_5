use bevy::color::palettes::css::GREEN;
use bevy::math::vec2;
use bevy::prelude::*;

use crate::path_finding::tiles::Tile;
use crate::path_finding::tiles::Direction;
use crate::screen::Screen;

use super::selection::SelectedTiles;
use super::tile_set::tile_coord_translation;
use super::tile_set::TileSet;
use super::tile_set::TILE_ANCHOR;

pub struct MapRenderingPlugin;

impl Plugin for MapRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, despawn_temporary_sprites)
            .add_systems(
                PostUpdate,
                show_selected_tiles.run_if(in_state(Screen::Playing)),
            );
    }
}

fn despawn_temporary_sprites(query: Query<Entity, With<TemporarySprite>>, mut commands: Commands) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component)]
pub struct TemporarySprite;

fn tile_to_camera(tile: Tile, layer: f32) -> Vec3 {
    tile_coord_translation(tile.x() as f32, tile.y() as f32, layer)
}

fn show_selected_tiles(
    mut commands: Commands,
    selected: Res<SelectedTiles>,
    tile_set: Res<TileSet>,
) {
    let edge_image = tile_set.get("edge");
    for tile in selected.tiles.iter().copied() {
        let border_edges = Direction::ROOK.iter().filter(|s| {
            let t = tile.step(**s);
            !selected.tiles.contains(&t)
        });
        for edge in border_edges {
            let scalar = match edge {
                Direction::North => Vec2::ONE,
                Direction::East => vec2(1., -1.),
                Direction::South => -Vec2::ONE,
                Direction::West => vec2(-1., 1.),
                _ => Vec2::ZERO,
            };
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        anchor: TILE_ANCHOR,
                        color: selected.color,
                        ..Default::default()
                    },
                    texture: edge_image.clone(),
                    transform: Transform {
                        translation: tile_to_camera(tile, 1.),
                        scale: scalar.extend(1.),
                        ..Default::default()
                    },
                    ..default()
                },
                TemporarySprite,
            ));
        }
    }
}
