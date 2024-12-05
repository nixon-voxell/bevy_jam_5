use bevy::{color::palettes::css, prelude::*};
use bevy_trauma_shake::TraumaCommands;

use crate::game::audio::sfx::PlaySfx;
use crate::game::tile_set::tile_coord_translation;
use crate::path_finding::find_all_within_distance_unweighted;
use crate::path_finding::tiles::Tile;
use crate::screen::playing::GameState;
use crate::screen::Screen;

use super::actors::{stats::Health, ActorTurnState, ClearUndoEvent, EnemyActor};
use super::inventory::{Inventory, Item};
use super::map::VillageMap;
use super::selection::{self, SelectedActor, SelectedTiles, SelectionEvent};
use super::vfx::{FireOneShotVfx, OneShotVfx};

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InventorySelection>().add_systems(
            Update,
            (
                show_attack_range,
                apply_item_effect.after(selection::set_selected_unit),
                deselect_inventory_on_click,
            )
                .chain()
                .run_if(in_state(GameState::BattleTurn).and_then(in_state(Screen::Playing))),
        );
    }
}

#[derive(Resource, Default)]
pub struct InventorySelection {
    pub selection: Option<(Item, Entity, usize)>,
    pub tile: Tile,
}

fn show_attack_range(
    q_inventories: Query<(Entity, &Inventory, &ActorTurnState), Changed<Inventory>>,
    mut selection_tiles: ResMut<SelectedTiles>,
    village_map: Res<VillageMap>,
    mut inventory_selection: ResMut<InventorySelection>,
) {
    for (entity, inventory, turn_state) in q_inventories.iter() {
        if turn_state.used_action {
            continue;
        }

        if let Some(item) = inventory.selected_item.and_then(|i| inventory.get(i)) {
            let Some(tile) = village_map.actors.locate(entity) else {
                continue;
            };

            let possible_action_tiles =
                find_all_within_distance_unweighted(tile, item.range, |t| {
                    item.directions.iter().copied().map(move |d| t.step(d))
                });

            selection_tiles.tiles = possible_action_tiles;
            selection_tiles.color = css::ORANGE.into();

            *inventory_selection = InventorySelection {
                selection: Some((item, entity, inventory.selected_item.unwrap())),
                tile,
            };
        }
    }
}

fn deselect_inventory_on_click(
    mut inventory_selection: ResMut<InventorySelection>,
    mut selection_events: EventReader<SelectionEvent>,
) {
    if selection_events.is_empty() == false {
        selection_events.clear();
        inventory_selection.selection = None;
    }
}

fn apply_item_effect(
    mut commands: Commands,
    mut q_inventories: Query<(&mut Inventory, &mut ActorTurnState)>,
    mut q_healths: Query<&mut Health>,
    q_enemy_units: Query<(), With<EnemyActor>>,
    mut village_map: ResMut<VillageMap>,
    selected_unit: Res<SelectedActor>,
    inventory_selection: Res<InventorySelection>,
    mut selection_events: EventReader<SelectionEvent>,
    mut clear_undo_event: EventWriter<ClearUndoEvent>,
    mut evw_oneshot_vfx: EventWriter<FireOneShotVfx>,
) {
    if selection_events.is_empty() {
        return;
    } else {
        selection_events.clear();
    }

    let Some((mut item, origin_entity, index)) = inventory_selection.selection else {
        return;
    };
    let Some(target_entity) = selected_unit.entity else {
        return;
    };
    let Some(target_tile) = village_map.actors.locate(target_entity) else {
        return;
    };
    let Ok((mut inventory, mut turn_state)) = q_inventories.get_mut(origin_entity) else {
        return;
    };

    if turn_state.used_action {
        return;
    }

    // Cannot apply negative effect on player units
    if q_enemy_units.contains(target_entity) == false && item.health_effect < 0 {
        return;
    }

    // Cannot apply positive effect on enemy units
    if q_enemy_units.contains(target_entity) && item.health_effect > 0 {
        return;
    }

    info!("Using item: {}", item.name);

    let possible_action_tiles =
        find_all_within_distance_unweighted(inventory_selection.tile, item.range, |t| {
            item.directions.iter().copied().map(move |d| t.step(d))
        });

    if possible_action_tiles.contains(&target_tile) {
        if let Ok(mut health) = q_healths.get_mut(target_entity) {
            if item.health_effect > 0 {
                health.value += item.health_effect as u32;
            } else {
                health.value = health
                    .value
                    .saturating_sub(item.health_effect.unsigned_abs());

                let mut tile_trans =
                    tile_coord_translation(target_tile.x() as f32, target_tile.y() as f32, 3.0);
                tile_trans.y += 100.0;

                evw_oneshot_vfx.send(FireOneShotVfx(
                    OneShotVfx::AttackFlash,
                    Transform::from_translation(tile_trans),
                ));
                evw_oneshot_vfx.send(FireOneShotVfx(
                    OneShotVfx::BloodSplash,
                    Transform::from_translation(tile_trans),
                ));
                commands.add_trauma(0.5);

                if health.value == 0 {
                    village_map.actors.remove_entity(target_entity);
                }
            }

            commands.trigger(PlaySfx::Key(item.sfx));

            println!("Successfully used item: {}", item.name);

            // Use up one item
            item.item_count = item.item_count.saturating_sub(1);
            if item.item_count > 0 {
                // Set back item if it is not used up yet.
                inventory.set(index, item);
            }

            turn_state.used_action = true;

            clear_undo_event.send(ClearUndoEvent);
        }
    }
}
