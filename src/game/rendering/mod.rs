pub mod extract_map;

use super::components::ArcherTower;
use super::map::VillageMap;

use super::picking::PickedTile;
use super::selection::SelectedActor;

use super::tile_set::tile_coord_translation;
use crate::path_finding::tiles::Tile;
use crate::screen::playing::GameState;
use crate::screen::Screen;

use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource, Default)]
pub struct ShowCoords {
    show_tile_coords: bool,
}

pub struct MapRenderingPlugin;

impl Plugin for MapRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShowCoords>()
            .init_resource::<TileTints>()
            .add_systems(
                Update,
                spawn_tile_coord_labels.run_if(in_state(Screen::Playing)),
            )
            .add_systems(
                PostUpdate,
                (set_arrow_tints.run_if(in_state(GameState::BattleTurn)),)
                    .run_if(in_state(Screen::Playing)),
            );
    }
}

#[derive(Component)]
pub struct TemporarySprite;

#[derive(Resource, Default)]
pub struct TileTints(pub HashMap<Tile, Color>);

fn tile_to_camera(tile: Tile, layer: f32) -> Vec3 {
    tile_coord_translation(tile.x() as f32, tile.y() as f32, layer)
}

fn spawn_tile_coord_labels(
    mut flag: Local<bool>,
    mut commands: Commands,
    map: Res<VillageMap>,
    layers: Res<ShowCoords>,
    query: Query<Entity, With<TemporarySprite>>,
) {
    if layers.show_tile_coords != *flag {
        if layers.show_tile_coords {
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
        } else {
            for entity in query.iter() {
                commands.entity(entity).despawn();
            }
        }
        *flag = layers.show_tile_coords;
    }
}

fn set_arrow_tints(
    selected: Res<SelectedActor>,
    village_map: Res<VillageMap>,
    query: Query<&Tile, With<ArcherTower>>,
    picked_tile: Res<PickedTile>,
    mut tile_tints: ResMut<TileTints>,
) {
    tile_tints.0.clear();
    let Some(selected_entity) = selected.entity else {
        return;
    };
    let Ok(tile) = query.get(selected_entity).copied() else {
        return;
    };

    let Some(picked_tile) = picked_tile.0 else {
        return;
    };

    let Some(mut line_iterator) = tile.get_line_through(picked_tile) else {
        return;
    };

    while let Some(cursor) = line_iterator
        .next()
        .filter(|&cursor| village_map.bounds().contains(cursor))
    {
        tile_tints
            .0
            .insert(cursor, bevy::color::palettes::tailwind::RED_400.into());
    }
}
