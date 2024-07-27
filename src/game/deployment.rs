use crate::screen::Screen;

use super::map::VillageMap;
use super::picking::TilePressedEvent;
use super::selection::SelectedTiles;
use super::selection::SelectedUnit;
use super::tile_set::tile_coord_translation;
use super::tile_set::TileSet;
use super::unit::player;
use super::unit_list::PlayerUnitList;
use bevy::color::palettes::css::GREEN;
use bevy::color::palettes::css::LIME;
use bevy::color::palettes::css::YELLOW;
use bevy::prelude::*;

pub fn deployment_setup(
    player_unit_list: Res<PlayerUnitList>,
    mut selected_unit: ResMut<SelectedUnit>,
    mut village_map: ResMut<VillageMap>,
) {
    selected_unit.entity = player_unit_list.0.get(0).copied();
    let size = village_map.size;
    let r = IRect::from_corners(
        IVec2::ZERO,
        IVec2 {
            x: size.x as i32,
            y: size.y as i32,
        },
    )
    .inflate(-3);
    for x in r.min.x..r.max.x {
        for y in r.min.y..r.max.y {
            village_map.deployment_zone.insert(IVec2::new(x, y));
        }
    }
}

pub fn deployment_zone_visualization(
    village_map: Res<VillageMap>,
    mut selected_tiles: ResMut<SelectedTiles>,
) {
    selected_tiles.tiles = village_map.deployment_zone.clone();
    selected_tiles.color = LIME.into();
}

pub fn is_deployment_ready(
    player_unit_list: Res<PlayerUnitList>,
    village_map: Res<VillageMap>,
) -> bool {
    for entity in player_unit_list.0.iter() {
        if village_map.object.locate(*entity).is_none() {
            return false;
        }
    }
    true
}

pub fn deploy_unit(
    mut events: EventReader<TilePressedEvent>,
    mut village_map: ResMut<VillageMap>,
    mut selected_unit: ResMut<SelectedUnit>,
    player_unit_list: Res<PlayerUnitList>,
    tile_set: Res<TileSet>,
    mut commands: Commands,
) {
    let Some(entity_to_deploy) = selected_unit.entity else {
        return;
    };
    if player_unit_list.0.contains(&entity_to_deploy) {
        if let Some(TilePressedEvent(target_tile)) = events.read().next() {
            if village_map.deployment_zone.contains(target_tile)
                && !village_map.object.is_occupied(*target_tile)
            {
                let translation =
                    tile_coord_translation(target_tile.x as f32, target_tile.y as f32, 2.0);
                commands.entity(entity_to_deploy).insert((
                    SpriteBundle {
                        sprite: Sprite {
                            anchor: super::tile_set::TILE_ANCHOR,
                            ..default()
                        },
                        transform: Transform::from_translation(translation),
                        texture: tile_set.get("human"),
                        ..default()
                    },
                    StateScoped(Screen::Playing),
                ));
                village_map.object.set(*target_tile, entity_to_deploy);
                if let Some(next_unit) = player_unit_list
                    .0
                    .iter()
                    .find(|entity| village_map.object.locate(**entity).is_none())
                {
                    println!("deployed: {entity_to_deploy:?}, next unit: {next_unit:?}");
                    selected_unit.set(*next_unit);
                }
            }
        }
    }
    events.clear()
}
