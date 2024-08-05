use super::components::GroundTileLayer;
use super::deployment::deploy_unit;

use super::map::VillageMap;
use super::picking::PickedTile;

use super::picking::TilePressedEvent;

use super::unit::EnemyUnit;

use super::unit::Movement;
use super::unit::UnitTurnState;

use crate::path_finding::tiles::Direction;
use crate::path_finding::tiles::Tile;
use crate::screen::playing::GameState;
use crate::screen::Screen;
use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::utils::HashSet;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedTiles>()
            .init_resource::<SelectionMap>()
            .init_resource::<SelectedUnit>()
            .add_event::<SelectionEvent>()
            .add_event::<ObjectPressedEvent>()
            .add_systems(
                Update,
                (
                    //show_selected_tiles.run_if(resource_changed::<SelectedTiles>),
                    color_selected_tiles.run_if(resource_changed::<SelectedTiles>),
                    set_selected_unit
                        .run_if(in_state(Screen::Playing))
                        .before(deploy_unit),
                    on_selection.after(set_selected_unit),
                    show_movement_range
                        .after(on_selection)
                        .run_if(not(in_state(GameState::Deployment)))
                        .run_if(resource_changed::<SelectedUnit>),
                )
                    .run_if(in_state(Screen::Playing)),
            );
    }
}

/// Current selected unit, can be Player controlled, enemy or a building
#[derive(Resource, Default)]
pub struct SelectedUnit {
    pub entity: Option<Entity>,
}

impl SelectedUnit {
    pub fn set(&mut self, entity: Entity) {
        self.entity = Some(entity);
    }
}

#[derive(Resource, Default)]
pub struct SelectedTiles {
    pub color: Color,
    pub tiles: HashSet<Tile>,
}

#[derive(Resource, Default)]
pub struct SelectionMap {
    pub borders: HashMap<Tile, Entity>,
    pub thick_borders: HashMap<Tile, Entity>,
}

fn color_selected_tiles(
    selected_tiles: Res<SelectedTiles>,
    mut query: Query<(&mut Sprite, &Tile), With<GroundTileLayer>>,
) {
    for (mut s, tile) in query.iter_mut() {
        s.color = if selected_tiles.tiles.contains(tile) {
            selected_tiles.color
        } else {
            Color::WHITE
        };
    }
}

pub fn show_movement_range(
    q_movements: Query<(&Movement, &UnitTurnState)>,
    q_enemies: Query<(), With<EnemyUnit>>,
    selected_unit: Res<SelectedUnit>,
    mut selected_tiles: ResMut<SelectedTiles>,
    village_map: Res<VillageMap>,
) {
    let Some(entity) = selected_unit.entity else {
        return;
    };
    let (Some(tile), Ok((movement, turn_state))) =
        (village_map.object.locate(entity), q_movements.get(entity))
    else {
        return;
    };

    if turn_state.used_move {
        selected_tiles.tiles.clear();
        return;
    }

    let tiles = village_map.flood(tile, movement.0, &Direction::ROOK, false);
    selected_tiles.tiles = tiles;
    match q_enemies.contains(entity) {
        true => selected_tiles.color = css::INDIAN_RED.into(),
        false => {
            // match turn_state.used_move
            selected_tiles.color = css::LIME.into();
        }
    }
}

#[derive(Event, Debug)]
pub enum SelectionEvent {
    Selected(Entity),
    Deselected(Entity),
}

#[derive(Event)]
pub struct DeselectedUnitEvent(pub Entity);

pub fn set_selected_unit(
    mouse_button: Res<ButtonInput<MouseButton>>,
    picked_tile: Res<PickedTile>,
    village_map: Res<VillageMap>,
    mut selected_unit: ResMut<SelectedUnit>,
    mut selection_event: EventWriter<SelectionEvent>,
) {
    if mouse_button.just_pressed(MouseButton::Left) {
        if let Some(tile) = picked_tile.0 {
            if let Some(new_selection) = village_map.object.get(tile) {
                if let Some(previous_selection) = selected_unit.entity {
                    if new_selection == previous_selection {
                        return;
                    }
                    selection_event.send(SelectionEvent::Deselected(previous_selection));
                }
                selection_event.send(SelectionEvent::Selected(new_selection));
                selected_unit.entity = Some(new_selection);
            }
        }
    }
}

#[derive(Component)]
pub struct SelectionMarkerSprite;

pub fn on_selection(
    mut commands: Commands,
    mut selection_events: EventReader<SelectionEvent>,
    query: Query<Entity, With<SelectionMarkerSprite>>,
    asset_server: Res<AssetServer>,
) {
    for selection_event in selection_events.read() {
        match selection_event {
            SelectionEvent::Selected(entity) => {
                if let Some(mut entity_commands) = commands.get_entity(*entity) {
                    entity_commands.with_children(|builder| {
                        builder.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    anchor: bevy::sprite::Anchor::BottomCenter,
                                    color: Color::WHITE,
                                    custom_size: Some(Vec2::splat(64.)),
                                    ..Default::default()
                                },
                                texture: asset_server.load("icons/selection_arrow.png"),
                                transform: Transform::from_xyz(0., 100., 0.1),
                                ..Default::default()
                            },
                            SelectionMarkerSprite,
                        ));
                    });
                }
            }
            SelectionEvent::Deselected(_) => {
                for entity in query.iter() {
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

#[derive(Event, Copy, Clone, PartialEq)]
pub struct ObjectPressedEvent(pub Entity);

pub fn dispatch_object_pressed(
    map: Res<VillageMap>,
    mut events: EventReader<TilePressedEvent>,
    mut dispatcher: EventWriter<ObjectPressedEvent>,
) {
    for TilePressedEvent(tile) in events.read().copied() {
        if let Some(entity) = map.object.get(tile) {
            dispatcher.send(ObjectPressedEvent(entity));
        }
    }
}
