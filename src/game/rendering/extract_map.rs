use bevy::app::Plugin;
use bevy::color::palettes::tailwind::YELLOW_300;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::render::Extract;
use bevy::render::RenderApp;
use bevy::sprite::extract_sprites;
use bevy::sprite::ExtractedSprite;
use bevy::sprite::ExtractedSprites;
use bevy::sprite::SpriteSystem;

use crate::game::actors::stats::Health;
use crate::game::components::ArcherTower;
use crate::game::constants;
use crate::game::constants::CURSOR_COLOR;
use crate::game::game_params::ReadGame;
use crate::game::map::VillageMap;
use crate::game::picking::PickedTile;
use crate::game::selection::SelectedActor;
use crate::game::selection::SelectedTiles;
use crate::game::tile_set::TileSet;
use crate::game::tile_set::TILE_ANCHOR;
use crate::path_finding::tiles::Tile;
use crate::path_finding::tiles::TileCorner;
use crate::path_finding::tiles::TileEdge;
use crate::path_finding::tiles::Tiled;
use crate::screen::playing::GameState;
use crate::ui::icon_set::IconSet;

use super::tile_to_camera;
use super::TileTints;

pub struct MapExtractionPlugin;

#[derive(Resource)]
struct MapEnt(Entity);

impl Plugin for MapExtractionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, |mut commands: Commands| {
            let e = commands
                .spawn(SpriteBundle {
                    ..Default::default()
                })
                .id();
            commands.insert_resource(MapEnt(e));
        });

        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(
                ExtractSchedule,
                (
                    extract_terrain,
                    extract_selected_tiles,
                    extract_arrow_sprites,
                    extract_tile_cursor,
                    extract_arrow_sprites,
                    extract_health,
                )
                    .in_set(SpriteSystem::ExtractSprites)
                    .after(extract_sprites),
            );
        }
    }
}

fn extract_terrain(
    mut commands: Commands,
    mut extracted_sprites: ResMut<ExtractedSprites>,
    tile_set: Extract<Res<TileSet>>,
    selected: Extract<Res<SelectedTiles>>,
    game_state: Extract<Res<State<GameState>>>,
    tints: Extract<Res<TileTints>>,
    game: Extract<ReadGame>,
    ent: Extract<Res<MapEnt>>,
) {
    if game.map.is_none() {
        return;
    }

    let state = game_state.get();
    let tint = |tile: Tile| {
        match state {
            // ..
            // ..
            GameState::Deployment => {
                if game.deployment_zone().contains(&tile) {
                    constants::DEPLOYMENT_ZONE_COLOR.into()
                } else {
                    Color::WHITE
                }
            }
            _ => {
                if selected.tiles.contains(&tile) {
                    selected.color
                } else if let Some(color) = tints.0.get(&tile) {
                    *color
                } else {
                    Color::WHITE
                }
            }
        }
    };

    let buildable = game.find_tiles_that_can_be_built_on();

    for (tile, terrain) in game.map().iter_terrain() {
        extracted_sprites.sprites.insert(
            commands.spawn_empty().id(),
            ExtractedSprite {
                color: tint(tile).into(),
                transform: Transform::from_translation(tile_to_camera(tile, 0.)).into(),
                rect: None,
                anchor: TILE_ANCHOR.as_vec(),
                original_entity: Some(ent.0),
                custom_size: None,
                image_handle_id: tile_set.get_terrain(terrain).id(),
                flip_x: false,
                flip_y: false,
            },
        );
        if buildable.contains(&tile) && *state == GameState::BuildingTurn {
            extracted_sprites.sprites.insert(
                commands.spawn_empty().id(),
                ExtractedSprite {
                    color: Color::WHITE.with_alpha(0.35).into(),
                    transform: Transform::from_translation(tile_to_camera(tile, 0.01)).into(),
                    rect: None,
                    anchor: TILE_ANCHOR.as_vec(),
                    original_entity: Some(ent.0),
                    custom_size: None,
                    image_handle_id: tile_set.get("block_blue").id(),
                    flip_x: false,
                    flip_y: false,
                },
            );
        }
    }
}

fn extract_selected_tiles(
    mut commands: Commands,
    mut extracted_sprites: ResMut<ExtractedSprites>,
    selected: Extract<Res<SelectedTiles>>,
    tile_set: Extract<Res<TileSet>>,
    ent: Extract<Res<MapEnt>>,
    maybe_village_map: Extract<Option<Res<VillageMap>>>,
) {
    let Some(village_map) = maybe_village_map.as_ref() else {
        return;
    };
    let edge_image = tile_set.get("edge");
    let ne_corner_image = tile_set.get("ne_corner");
    let se_corner_image = tile_set.get("se_corner");
    for tile in selected
        .tiles
        .iter()
        .copied()
        .filter(|tile| village_map.contains_tile(*tile))
    {
        let border_edges = TileEdge::ALL
            .iter()
            .filter(|e| {
                let t = tile.step(e.direction());
                !selected.tiles.contains(&t)
            })
            .copied();
        for edge in border_edges {
            let scalar = match edge {
                TileEdge::North => vec2(-1., 1.),
                TileEdge::East => Vec2::ONE,
                TileEdge::South => vec2(1., -1.),
                TileEdge::West => -Vec2::ONE,
            };

            extracted_sprites.sprites.insert(
                commands.spawn_empty().id(),
                ExtractedSprite {
                    color: selected.color.into(),
                    transform: Transform {
                        translation: tile_to_camera(tile, 1.),
                        scale: scalar.extend(1.),
                        ..Default::default()
                    }
                    .into(),
                    rect: None,
                    anchor: TILE_ANCHOR.as_vec(),
                    original_entity: Some(ent.0),
                    custom_size: None,
                    image_handle_id: edge_image.id(),
                    flip_x: false,
                    flip_y: false,
                },
            );
        }

        let corners = TileCorner::ALL.iter().filter(|c| {
            let t = tile.step(c.direction());
            !selected.tiles.contains(&t)
        });
        for corner in corners {
            let (image, scalar) = match corner {
                TileCorner::NorthEast => (&ne_corner_image, Vec2::ONE),
                TileCorner::SouthEast => (&se_corner_image, Vec2::ONE),
                TileCorner::SouthWest => (&ne_corner_image, -Vec2::ONE),
                TileCorner::NorthWest => (&se_corner_image, -Vec2::ONE),
            };

            extracted_sprites.sprites.insert(
                commands.spawn_empty().id(),
                ExtractedSprite {
                    color: selected.color.into(),
                    transform: Transform {
                        translation: tile_to_camera(tile, 1.),
                        scale: scalar.extend(1.),
                        ..Default::default()
                    }
                    .into(),
                    rect: None,
                    anchor: TILE_ANCHOR.as_vec(),
                    original_entity: Some(ent.0),
                    custom_size: None,
                    image_handle_id: image.id(),
                    flip_x: false,
                    flip_y: false,
                },
            );
        }
    }
}

fn extract_tile_cursor(
    mut commands: Commands,
    mut extracted_sprites: ResMut<ExtractedSprites>,
    picked_tile: Extract<Res<PickedTile>>,
    tile_set: Extract<Res<TileSet>>,
    ent: Extract<Res<MapEnt>>,
) {
    let image = tile_set.get("border");
    if let Some(tile) = picked_tile.0 {
        extracted_sprites.sprites.insert(
            commands.spawn_empty().id(),
            ExtractedSprite {
                color: CURSOR_COLOR.into(),
                transform: Transform {
                    translation: tile_to_camera(tile, 1.),
                    ..Default::default()
                }
                .into(),
                rect: None,
                anchor: TILE_ANCHOR.as_vec(),
                original_entity: Some(ent.0),
                custom_size: None,
                image_handle_id: image.id(),
                flip_x: false,
                flip_y: false,
            },
        );
    }
}

fn extract_arrow_sprites(
    mut commands: Commands,
    mut extracted_sprites: ResMut<ExtractedSprites>,
    selected: Extract<Res<SelectedActor>>,
    village_map: Extract<Option<Res<VillageMap>>>,
    query: Extract<Query<&Tile, With<ArcherTower>>>,
    asset_server: Extract<Res<AssetServer>>,
    picked_tile: Extract<Res<PickedTile>>,
    ent: Extract<Res<MapEnt>>,
) {
    let Some(village_map) = village_map.as_ref() else {
        return;
    };
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
        TileEdge::North => (true, false),
        TileEdge::East => (false, false),
        TileEdge::South => (false, true),
        TileEdge::West => (true, true),
    };

    while let Some(cursor) = line_iterator
        .next()
        .filter(|&cursor| village_map.bounds().contains(cursor))
    {
        if village_map.actors.is_occupied(cursor) {
            break;
        }
        let make_arrow_sprite_bundle =
            |tile: Tile, height: f32, layer: f32, color: Color| ExtractedSprite {
                color: color.into(),
                transform: Transform {
                    translation: tile_to_camera(tile, layer) + height * Vec3::Y,
                    scale: Vec3::new(2., 2., 1.),
                    ..default()
                }
                .into(),
                rect: None,
                anchor: TILE_ANCHOR.as_vec(),
                original_entity: Some(ent.0),
                custom_size: None,
                image_handle_id: asset_server.load("tiles/arrow.png").id(),
                flip_x,
                flip_y,
            };

        // arrow sprite
        extracted_sprites.sprites.insert(
            commands.spawn_empty().id(),
            make_arrow_sprite_bundle(cursor, 45., 1.2, Color::WHITE),
        );

        // shadow sprite
        extracted_sprites.sprites.insert(
            commands.spawn_empty().id(),
            make_arrow_sprite_bundle(cursor, 0., 1.1, Color::srgba(0.2, 0.2, 0.2, 0.8)),
        );
    }
}

const HEART_SIZE: Vec2 = Vec2::new(40.0, 40.0);
const HEART_GAP: f32 = 10.0;

fn extract_health(
    mut commands: Commands,
    mut extracted_sprites: ResMut<ExtractedSprites>,
    query: Extract<Query<(Entity, &Health)>>,
    icon_set: Extract<Res<IconSet>>,
    map: Extract<Option<Res<VillageMap>>>,
    ent: Extract<Res<MapEnt>>,
) {
    let Some(map) = map.as_ref() else {
        return;
    };
    for (entity, health) in query.iter() {
        let health_width = (HEART_SIZE.x + HEART_GAP) * health.max as f32 - HEART_GAP;
        let x_offset = -0.5 * (health_width - HEART_SIZE.x);
        let inner_panel_size = vec2(health_width + 2. * HEART_GAP, HEART_SIZE.y + HEART_GAP);
        let outer_panel_size = inner_panel_size + 8.;

        let Some(tile) = map.actors.locate(entity) else {
            continue;
        };

        let translation = tile_to_camera(tile, 10.) + 250. * Vec3::Y;

        let h: Handle<Image> = Handle::default();

        extracted_sprites.sprites.insert(
            commands.spawn_empty().id(),
            ExtractedSprite {
                color: YELLOW_300.into(),
                transform: Transform::from_translation(translation).into(),
                rect: None,
                anchor: TILE_ANCHOR.as_vec(),
                original_entity: Some(ent.0),
                custom_size: Some(outer_panel_size),
                image_handle_id: h.id(),
                flip_x: false,
                flip_y: false,
            },
        );

        extracted_sprites.sprites.insert(
            commands.spawn_empty().id(),
            ExtractedSprite {
                color: Color::BLACK.into(),
                transform: Transform::from_translation(translation + Vec3::Z * 0.1).into(),
                rect: None,
                anchor: TILE_ANCHOR.as_vec(),
                original_entity: Some(ent.0),
                custom_size: Some(inner_panel_size),
                image_handle_id: h.id(),
                flip_x: false,
                flip_y: false,
            },
        );

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

            extracted_sprites.sprites.insert(
                commands.spawn_empty().id(),
                ExtractedSprite {
                    color: color.into(),
                    transform: Transform::from_translation(translation).into(),
                    rect: None,
                    anchor: TILE_ANCHOR.as_vec(),
                    original_entity: Some(ent.0),
                    custom_size: Some(HEART_SIZE),
                    image_handle_id: icon_set.get("heart").id(),
                    flip_x: false,
                    flip_y: false,
                },
            );
        }
    }
}
