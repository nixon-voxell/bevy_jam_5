use super::deployment::deploy_unit;
use super::map::VillageMap;
use super::selection::dispatch_object_pressed;
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
        app.init_resource::<PickedPointWorldCamera>()
            .init_resource::<PickedPointWorld>()
            .init_resource::<PickedTile>()
            .add_event::<TilePressedEvent>()
            .add_systems(
                Update,
                (
                    pointer_coords_to_world_camera_coords,
                    world_camera_picked_point_to_tile_coords,
                    pick_tile,
                    dispatch_pressed_tile,
                    dispatch_object_pressed,
                    deploy_unit.run_if(in_state(GameState::Deployment)),
                )
                    .chain()
                    .run_if(in_state(Screen::Playing)),
            );
    }
}

#[derive(Component)]
pub struct PickableTile;

#[derive(Resource, Default, Debug, PartialEq)]
pub struct PickedPointWorldCamera(pub Option<Vec2>);

// The position of the pointer in fractional world tile coordinates.
#[derive(Resource, Default, Debug, PartialEq)]
pub struct PickedPointWorld(pub Option<Vec2>);

// The tile currently hovered by the mouse pointer
#[derive(Resource, Default, Debug, Eq, PartialEq)]
pub struct PickedTile(pub Option<Tile>);

/// If the pointer is over the game window,
/// set `WorldCameraPick` to the pointers position in world camera coords,
/// otherwise set `WorldCameraPick` to None
pub fn pointer_coords_to_world_camera_coords(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    mut picked_point: ResMut<PickedPointWorldCamera>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    picked_point.set_if_neq(PickedPointWorldCamera(
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            Some(world_position)
        } else {
            None
        },
    ));
}

/// Converts world camera coords to fractional (possibly negative) world tile coordinates
pub fn world_camera_picked_point_to_tile_coords(
    camera_point: Res<PickedPointWorldCamera>,
    mut world_point: ResMut<PickedPointWorld>,
) {
    world_point.set_if_neq(PickedPointWorld(camera_point.0.map(|p| {
        let tile_width = TILE_WIDTH;
        let tile_half_height = TILE_HALF_HEIGHT;
        let x =
            (-tile_half_height * p.x - (tile_width / 2.0) * p.y) / (tile_width * tile_half_height);
        let y =
            (tile_half_height * p.x - (tile_width / 2.0) * p.y) / (tile_width * tile_half_height);
        Vec2 { x, y }
    })));
}

/// The tile in the world currently hovered by the pointer
pub fn pick_tile(
    picked_point: Res<PickedPointWorld>,
    village_map: Res<VillageMap>,
    mut picked_tile: ResMut<PickedTile>,
) {
    picked_tile.set_if_neq(PickedTile(
        picked_point
            .0
            .map(|point| Tile::from(point))
            .filter(|tile| village_map.contains_tile(*tile)),
    ));
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
