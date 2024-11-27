use bevy::app::Plugin;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::render::Extract;
use bevy::render::RenderApp;
use bevy::sprite::extract_sprites;
use bevy::sprite::ExtractedSprite;
use bevy::sprite::ExtractedSprites;
use bevy::sprite::SpriteSystem;

use crate::game::actors::stats::Health;
use crate::game::actors::stats::Movement;
use crate::game::actors::EnemyActor;
use crate::game::actors::PlayerActor;
use crate::game::actors::Structure;
use crate::game::constants;
use crate::game::game_params::Game;
use crate::game::game_params::ReadGame;
use crate::game::map::VillageMap;
use crate::game::selection::SelectedTiles;
use crate::game::tile_set::TileSet;
use crate::game::tile_set::TILE_ANCHOR;
use crate::path_finding::tiles::Tile;
use crate::screen::playing::GameState;

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
                (extract_terrain)
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
    let h: Handle<Image> = Handle::default();
    extracted_sprites.sprites.insert(
        commands.spawn_empty().id(),
        ExtractedSprite {
            transform: GlobalTransform::default(),
            color: Color::WHITE.into(),
            rect: None,
            custom_size: Some(Vec2::new(100., 100.)),
            image_handle_id: h.id(),
            flip_x: false,
            flip_y: false,
            anchor: Vec2::ZERO,
            original_entity: Some(ent.0),
        },
    );

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
