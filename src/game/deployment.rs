use super::map::VillageMap;
use super::selection::SelectedTiles;
use super::selection::SelectedUnit;
use super::unit_list::PlayerUnitList;
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
    .inflate(-4);
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
    selected_tiles.color = YELLOW.into();
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
