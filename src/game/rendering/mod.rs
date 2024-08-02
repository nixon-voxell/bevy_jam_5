use super::constants::CURSOR_COLOR;
use super::map::VillageMap;
use super::picking::PickedTile;
use super::selection::SelectedTiles;
use super::tile_set::tile_coord_translation;
use super::tile_set::TileSet;
use super::tile_set::TILE_ANCHOR;
use crate::path_finding::tiles::Corner;
use crate::path_finding::tiles::Edge;
use crate::path_finding::tiles::Tile;
use crate::screen::Screen;
use bevy::color::palettes::tailwind::YELLOW_400;
use bevy::color::palettes::tailwind::YELLOW_900;
use bevy::color::palettes::tailwind::YELLOW_950;
use bevy::math::vec2;
use bevy::prelude::*;

#[derive(Resource)]
pub struct ShowLayers {
    show_selected_area: bool,
    show_tile_coords: bool,
}

impl Default for ShowLayers {
    fn default() -> Self {
        Self {
            show_selected_area: true,
            show_tile_coords: false,
        }
    }
}

pub struct MapRenderingPlugin;

impl Plugin for MapRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShowLayers>()
            .add_systems(First, despawn_temporary_sprites)
            .add_systems(
                Update,
                spawn_tile_coord_labels
                    .run_if(in_state(Screen::Playing))
                    .run_if(|layers: Res<ShowLayers>| layers.show_tile_coords),
            )
            .add_systems(
                PostUpdate,
                (
                    spawn_selected_tiles
                        .run_if(|layers: Res<ShowLayers>| layers.show_selected_area),
                    spawn_tile_cursor,
                )
                    .run_if(in_state(Screen::Playing)),
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

fn spawn_selected_tiles(
    mut commands: Commands,
    selected: Res<SelectedTiles>,
    tile_set: Res<TileSet>,
) {
    let edge_image = tile_set.get("edge");
    let nw_corner_image = tile_set.get("nw_corner");
    let ne_corner_image = tile_set.get("ne_corner");
    for tile in selected.tiles.iter().copied() {
        let border_edges = Edge::ALL
            .iter()
            .filter(|e| {
                let t = tile.step(e.direction());
                !selected.tiles.contains(&t)
            })
            .copied();
        for edge in border_edges {
            let scalar = match edge {
                Edge::North => Vec2::ONE,
                Edge::East => vec2(1., -1.),
                Edge::South => -Vec2::ONE,
                Edge::West => vec2(-1., 1.),
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

        let corners = Corner::ALL.iter().filter(|c| {
            let t = tile.step(c.direction());
            !selected.tiles.contains(&t)
        });
        for corner in corners {
            let (image, scalar) = match corner {
                Corner::NorthEast => (&ne_corner_image, Vec2::ONE),
                Corner::SouthEast => (&nw_corner_image, -Vec2::ONE),
                Corner::SouthWest => (&ne_corner_image, -Vec2::ONE),
                Corner::NorthWest => (&nw_corner_image, Vec2::ONE),
            };
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        anchor: TILE_ANCHOR,
                        color: selected.color,
                        ..Default::default()
                    },
                    texture: image.clone(),
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

fn spawn_tile_coord_labels(mut commands: Commands, map: Res<VillageMap>) {
    for tile in map.bounds() {
        commands.spawn((
            Text2dBundle {
                text: Text::from_section(
                    format!("{},{}", tile.x(), tile.y()),
                    TextStyle {
                        font_size: 30.,
                        ..Default::default()
                    },
                ),
                transform: Transform {
                    translation: tile_to_camera(tile, 5.),
                    ..Default::default()
                },

                ..Default::default()
            },
            TemporarySprite,
        ));
    }
}

fn spawn_tile_cursor(mut commands: Commands, picked_tile: Res<PickedTile>, tile_set: Res<TileSet>) {
    let image = tile_set.get("border");
    if let Some(tile) = picked_tile.0 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    anchor: TILE_ANCHOR,
                    color: CURSOR_COLOR.into(),
                    ..Default::default()
                },
                texture: image.clone(),
                transform: Transform {
                    translation: tile_to_camera(tile, 1.1),
                    ..Default::default()
                },
                ..default()
            },
            TemporarySprite,
        ));
    }
}
