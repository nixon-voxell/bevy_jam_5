use super::components::ArcherTower;
use super::components::GroundTileLayer;
use super::constants;
use super::constants::CURSOR_COLOR;
use super::map::VillageMap;
use super::picking::PickableTile;
use super::picking::PickedTile;
use super::selection::SelectedTiles;
use super::selection::SelectedUnit;
use super::tile_set::tile_coord_translation;
use super::tile_set::TileSet;
use super::tile_set::TILE_ANCHOR;
use super::unit::Health;
use crate::path_finding::tiles::Corner;
use crate::path_finding::tiles::Edge;
use crate::path_finding::tiles::Tile;
use crate::screen::playing::GameState;
use crate::screen::Screen;
use crate::ui::icon_set::IconSet;

use bevy::color::palettes::tailwind::TEAL_300;
use bevy::color::palettes::tailwind::YELLOW_300;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::utils::HashMap;

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
            .init_resource::<TileTints>()
            .add_systems(PreUpdate, cleanup)
            .add_systems(
                Update,
                spawn_tile_coord_labels
                    .run_if(in_state(Screen::Playing))
                    .run_if(|layers: Res<ShowLayers>| layers.show_tile_coords),
            )
            .add_systems(
                PostUpdate,
                (
                    spawn_arrow_sprites.run_if(in_state(GameState::BattleTurn)),
                    draw_terrain,
                    spawn_selected_tiles
                        .run_if(|layers: Res<ShowLayers>| layers.show_selected_area),
                    spawn_tile_cursor,
                    draw_health,
                )
                    .run_if(in_state(Screen::Playing)),
            );
    }
}

fn cleanup(
    query: Query<Entity, With<TemporarySprite>>,
    mut commands: Commands,
    mut tile_tints: ResMut<TileTints>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    tile_tints.0.clear();
}

#[derive(Component)]
pub struct TemporarySprite;

#[derive(Resource, Default)]
pub struct TileTints(pub HashMap<Tile, Color>);

fn tile_to_camera(tile: Tile, layer: f32) -> Vec3 {
    tile_coord_translation(tile.x() as f32, tile.y() as f32, layer)
}

fn spawn_selected_tiles(
    mut commands: Commands,
    selected: Res<SelectedTiles>,
    tile_set: Res<TileSet>,
) {
    let edge_image = tile_set.get("edge");
    let ne_corner_image = tile_set.get("ne_corner");
    let se_corner_image = tile_set.get("se_corner");
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
                Edge::North => vec2(-1., 1.),
                Edge::East => Vec2::ONE,
                Edge::South => vec2(1., -1.),
                Edge::West => -Vec2::ONE,
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
                Corner::SouthEast => (&se_corner_image, Vec2::ONE),
                Corner::SouthWest => (&ne_corner_image, -Vec2::ONE),
                Corner::NorthWest => (&se_corner_image, -Vec2::ONE),
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

fn spawn_arrow_sprites(
    mut commands: Commands,
    selected: Res<SelectedUnit>,
    village_map: Res<VillageMap>,
    query: Query<&Tile, With<ArcherTower>>,
    asset_server: Res<AssetServer>,
    picked_tile: Res<PickedTile>,
    mut tile_tints: ResMut<TileTints>,
) {
    let Some(selected_entity) = selected.entity else {
        return;
    };
    let Ok(tile) = query.get(selected_entity).copied() else {
        return;
    };

    let Some(picked_tile) = picked_tile.0 else {
        return;
    };

    let Some(edge) = tile.find_direction_edge(picked_tile) else {
        return;
    };

    let Some(mut line_iterator) = tile.get_line_through(picked_tile) else {
        return;
    };

    let (flip_x, flip_y) = match edge {
        Edge::North => (true, false),
        Edge::East => (false, false),
        Edge::South => (false, true),
        Edge::West => (true, true),
    };

    while let Some(cursor) = line_iterator
        .next()
        .filter(|&cursor| village_map.bounds().contains(cursor))
    {
        tile_tints
            .0
            .insert(cursor, bevy::color::palettes::tailwind::RED_400.into());
        if village_map.object.is_occupied(cursor) {
            break;
        }
        let make_arrow_sprite_bundle = |tile: Tile, height: f32, layer: f32, color: Color| {
            (
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        flip_x,
                        flip_y,
                        ..Default::default()
                    },
                    texture: asset_server.load("tiles/arrow.png"),
                    transform: Transform {
                        translation: tile_to_camera(tile, layer) + height * Vec3::Y,
                        scale: Vec3::new(2., 2., 1.),
                        ..default()
                    },
                    ..default()
                },
                TemporarySprite,
            )
        };

        // arrow sprite
        commands.spawn(make_arrow_sprite_bundle(cursor, 45., 1.2, Color::WHITE));

        // shadow sprite
        commands.spawn(make_arrow_sprite_bundle(
            cursor,
            0.,
            1.1,
            Color::srgba(0.2, 0.2, 0.2, 0.8),
        ));
    }
}

fn draw_terrain(
    mut commands: Commands,
    village_map: Res<VillageMap>,
    tile_set: Res<TileSet>,
    selected: Res<SelectedTiles>,
    game_state: Res<State<GameState>>,
    tints: Res<TileTints>,
) {
    let state = game_state.get();
    let tint = |tile: Tile| {
        match state {
            // ..
            // ..
            GameState::Deployment => {
                if village_map.deployment_zone.contains(&tile) {
                    constants::DEPLOYMENT_ZONE_COLOR.into()
                } else {
                    Color::WHITE
                }
            }
            _ => {
                if selected.tiles.contains(&tile) {
                    selected.color
                } else if let Some(color) = tints.0.get(&tile) {
                    return *color;
                } else {
                    Color::WHITE
                }
            }
        }
    };

    for (tile, terrain) in village_map.iter_terrain() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: tint(tile),
                    anchor: TILE_ANCHOR,
                    ..Default::default()
                },
                texture: tile_set.get_terrain(terrain),
                transform: Transform::from_translation(tile_to_camera(tile, 0.)),
                ..default()
            },
            tile,
            PickableTile,
            GroundTileLayer,
            TemporarySprite,
        ));
    }
}

const HEART_SIZE: Vec2 = Vec2::new(40.0, 40.0);
const HEART_GAP: f32 = 10.0;

pub fn draw_health(
    query: Query<(Entity, &Health)>,
    icon_set: Res<IconSet>,
    mut commands: Commands,
    map: Res<VillageMap>,
) {
    for (entity, health) in query.iter() {
        // if health.value == health.max {
        let health_width = (HEART_SIZE.x + HEART_GAP) * health.max as f32 - HEART_GAP;
        let x_offset = -0.5 * (health_width - HEART_SIZE.x);

        let inner_panel_size = vec2(health_width + 2. * HEART_GAP, HEART_SIZE.y + HEART_GAP);
        let outer_panel_size = inner_panel_size + 8.;

        //     continue;
        // }

        let Some(tile) = map.object.locate(entity) else {
            continue;
        };
        // println!("{entity:?} -> {health:?} -> {tile:?}");
        // let start_x = (-hs(health.max) + HEART_SIZE.x) * 0.5;

        let translation = tile_to_camera(tile, 10.) + 250. * Vec3::Y;

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: YELLOW_300.into(),
                    custom_size: Some(outer_panel_size),
                    ..default()
                },
                transform: Transform::from_translation(translation),
                ..Default::default()
            },
            TemporarySprite,
        ));

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(inner_panel_size),
                    ..default()
                },
                transform: Transform::from_translation(translation + Vec3::Z * 0.1),
                ..Default::default()
            },
            TemporarySprite,
        ));

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(HEART_SIZE),
                    ..default()
                },
                transform: Transform::from_translation(translation),
                ..Default::default()
            },
            TemporarySprite,
        ));
        for index in 0..health.max {
            let indexf = index as f32;

            let color = match index < health.value {
                true => Srgba::RED,
                false => Srgba::gray(0.3),
            };

            let translation = translation
                + x_offset * Vec3::X
                + (HEART_SIZE.x + HEART_GAP) * indexf * Vec3::X
                + Vec3::Z * 0.2;

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: color.into(),
                        custom_size: Some(HEART_SIZE),
                        ..default()
                    },
                    texture: icon_set.get("heart"),
                    transform: Transform::from_translation(translation),
                    ..default()
                },
                TemporarySprite,
            ));
        }
    }
}