use super::deployment::deploy_unit;
use super::level::TileBorder;
use super::selection::dispatch_object_pressed;
use super::selection::SelectionMap;
use super::tile_set::TILE_HALF_HEIGHT;
use super::tile_set::TILE_WIDTH;
use crate::path_finding::tiles::Tile;
use crate::screen::playing::GameState;
use crate::screen::Screen;
use bevy::app::Plugin;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PickedTileEntities>()
            .init_resource::<PickedTile>()
            .init_resource::<PickedPoint>()
            .add_event::<TilePressedEvent>()
            .add_systems(
                Update,
                (
                    find_picked_point,
                    pick_tile,
                    show_border_on_tile_pick,
                    dispatch_pressed_tile,
                    dispatch_object_pressed,
                    deploy_unit.run_if(in_state(GameState::Deployment)),
                )
                    .chain()
                    .run_if(in_state(Screen::Playing)),
            );
    }
}

pub fn pick_tile(
    picked_point: Res<PickedPoint>,
    mut picked_tile_entity: ResMut<PickedTileEntities>,
    mut picked_tile: ResMut<PickedTile>,
    tiles_query: Query<(Entity, &GlobalTransform, &Tile), With<PickableTile>>,
) {
    let mut picked_set = false;

    picked_tile_entity.0.clear();

    if let Some(point) = picked_point.0 {
        for (e, _, _, map_pos) in tiles_query
            .iter()
            .map(|(e, t, &map_pos)| {
                (
                    e,
                    (point - t.translation().xy()).abs(),
                    t.translation().z,
                    map_pos,
                )
            })
            .filter(|(_, r, _, _)| {
                is_point_in_triangle(r.x, r.y, 0.5 * TILE_WIDTH, TILE_HALF_HEIGHT)
            })
        {
            // sprite_query
            //     .get_mut(e)
            //     .map(|mut sprite| sprite.color = Color::srgb(1., 0., 0.))
            //     .ok();
            picked_tile_entity.0.push(e);

            picked_tile.0 = Some(map_pos);
            picked_set = true;
        }
    }

    if !picked_set {
        picked_tile.0 = None;
    }
}

pub fn show_border_on_tile_pick(
    mut previous: Local<Option<Tile>>,
    picked_tile: Res<PickedTile>,
    mut query: Query<&mut Visibility, With<TileBorder>>,
    selection_map: Res<SelectionMap>,
) {
    if let Some(tile) = picked_tile.0 {
        if Some(tile) != *previous {
            if let Some(prev_ent) = previous
                .and_then(|prev_tile| selection_map.borders.get(&prev_tile))
                .copied()
            {
                if let Ok(mut v) = query.get_mut(prev_ent) {
                    v.set_if_neq(Visibility::Hidden);
                }
            }
            if let Some(entity) = selection_map.borders.get(&tile) {
                if let Ok(mut v) = query.get_mut(*entity) {
                    v.set_if_neq(Visibility::Visible);
                }
            }
        }
    } else {
        for mut v in query.iter_mut() {
            v.set_if_neq(Visibility::Hidden);
        }
    }
    *previous = picked_tile.0;
}

#[derive(Component)]
pub struct PickableTile;

#[derive(Resource, Default, Debug)]
pub struct PickedTileEntities(pub Vec<Entity>);

#[derive(Resource, Default, Debug)]
pub struct PickedTile(pub Option<Tile>);

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

#[derive(Event, Debug, Copy, Clone)]
pub struct TilePressedEvent(pub Tile);

pub fn dispatch_pressed_tile(
    picked_tile: Res<PickedTile>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut tile_pressed_event: EventWriter<TilePressedEvent>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        if let Some(picked_tile) = picked_tile.0 {
            tile_pressed_event.send(TilePressedEvent(picked_tile));
        }
    }
}

fn is_point_in_triangle(x: f32, y: f32, w: f32, h: f32) -> bool {
    if x < 0.0 || y < 0.0 {
        return false;
    }
    y <= h - (h / w) * x
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct PickState {
    hovered: Option<Tile>,
    pressed: Option<Tile>,
}

#[derive(Resource, Default, Debug)]
pub struct PickStatus {
    previous: PickState,
    current: PickState,
}

pub fn update_pick_status(
    mut pickstatus: ResMut<PickStatus>,
    mouse_state: ResMut<ButtonInput<MouseButton>>,
    picked_tile: Res<PickedTile>,
) {
    pickstatus.previous = pickstatus.current;

    pickstatus.current.hovered = picked_tile.0;

    if mouse_state.just_released(MouseButton::Left) {
        pickstatus.current.pressed = None;
    }

    if mouse_state.just_pressed(MouseButton::Left) {
        pickstatus.current.pressed = picked_tile.0;
    }
}
