use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::screen::playing::GameState;
use crate::screen::Screen;

use super::map::VillageMap;
use super::tile_set::TILE_HALF_HEIGHT;
use super::tile_set::TILE_WIDTH;

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PickedTileEntities>()
            .init_resource::<PickedTile>()
            .init_resource::<PickedPoint>()
            .add_systems(
                Update,
                (find_picked_point, pick_tile)
                    .chain()
                    .run_if(in_state(Screen::Playing)),
            );
    }
}

pub fn pick_tile(
    picked_point: Res<PickedPoint>,
    mut picked_tile_entity: ResMut<PickedTileEntities>,
    mut picked_tile: ResMut<PickedTile>,
    village_map: Res<VillageMap>,
    tiles_query: Query<(Entity, &GlobalTransform), With<PickableTile>>,
    mut sprite_query: Query<&mut Sprite>,
) {
    let mut picked_set = false;
    for previous in picked_tile_entity.0.drain(..) {
        sprite_query
            .get_mut(previous)
            .map(|mut sprite| sprite.color = Color::WHITE)
            .ok();
    }

    if let Some(point) = picked_point.0 {
        for (e, ..) in tiles_query
            .iter()
            .map(|(e, t)| (e, (point - t.translation().xy()).abs(), t.translation().z))
            .filter(|(_, r, _)| is_point_in_triangle(r.x, r.y, 0.5 * TILE_WIDTH, TILE_HALF_HEIGHT))
        {
            sprite_query
                .get_mut(e)
                .map(|mut sprite| sprite.color = Color::srgb(1., 0., 0.))
                .ok();
            picked_tile_entity.0.push(e);

            if let Some(tile) = village_map.ground.locate(e) {
                picked_tile.0 = Some(tile);
                picked_set = true;
            }
        }
    }

    if !picked_set {
        picked_tile.0 = None;
    }
}

#[derive(Component)]
pub struct PickableTile;

#[derive(Resource, Default, Debug)]
pub struct PickedTileEntities(pub Vec<Entity>);

#[derive(Resource, Default, Debug)]
pub struct PickedTile(pub Option<IVec2>);

#[derive(Resource, Default)]
pub struct PickedPoint(pub Option<Vec2>);

pub fn find_picked_point(
    mut picked_point: ResMut<PickedPoint>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        picked_point.0 = Some(world_position);
    } else {
        picked_point.0 = None;
    }
}

fn is_point_in_triangle(x: f32, y: f32, w: f32, h: f32) -> bool {
    if x < 0.0 || y < 0.0 {
        return false;
    }
    y <= h - (h / w) * x
}
