use bevy::prelude::*;
use bevy_trauma_shake::TraumaCommands;

use crate::{
    game::{
        tile_set::{tile_coord_translation, TILE_ANCHOR},
        unit::{
            enemy::{ClawMarkBundle, CLAW_ANIM_DURATAION},
            spawn::DespawnAnimation,
        },
    },
    path_finding::find_all_within_distance_unweighted,
    screen::{playing::GameState, Screen},
    ui::icon_set::IconSet,
};

use super::{inventory::Inventory, map::VillageMap, selection::SelectionEvent, unit::Health};

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            apply_item_effect
                .run_if(in_state(GameState::BattleTurn).and_then(in_state(Screen::Playing))),
        );
    }
}

fn apply_item_effect(
    mut commands: Commands,
    mut q_inventories: Query<&mut Inventory>,
    mut q_healths: Query<&mut Health>,
    mut selection_events: EventReader<SelectionEvent>,
    mut prev_selection: Local<Option<Entity>>,
    village_map: Res<VillageMap>,
    icon_set: Res<IconSet>,
) {
    for selection_event in selection_events.read() {
        match selection_event {
            SelectionEvent::Selected(entity) => {
                let Some(prev_entity) = *prev_selection else {
                    return;
                };

                let Ok(mut inventory) = q_inventories.get_mut(prev_entity) else {
                    return;
                };

                let (Some(prev_tile), Some(curr_tile)) = (
                    village_map.object.locate(prev_entity),
                    village_map.object.locate(*entity),
                ) else {
                    return;
                };

                if let Some(index) = inventory.selected_item {
                    if let Some(mut item) = inventory.take(index) {
                        println!("Using item: {}", item.name);
                        // Use up one item
                        item.item_count = item.item_count.saturating_sub(1);

                        let possible_action_tiles =
                            find_all_within_distance_unweighted(prev_tile, item.range, |t| {
                                item.directions.iter().copied().map(move |d| t + d)
                            });

                        if possible_action_tiles.contains(&curr_tile) {
                            if let Ok(mut health) = q_healths.get_mut(*entity) {
                                if item.health_effect > 0 {
                                    health.0 += item.health_effect as u32;
                                } else {
                                    health.0 =
                                        health.0.saturating_sub(item.health_effect.unsigned_abs());

                                    let translation = tile_coord_translation(
                                        curr_tile.x as f32,
                                        curr_tile.y as f32,
                                        3.0,
                                    );
                                    commands.spawn(ClawMarkBundle {
                                        sprite: SpriteBundle {
                                            sprite: Sprite {
                                                anchor: TILE_ANCHOR,
                                                ..default()
                                            },
                                            texture: icon_set.get("claw_mark"),
                                            ..default()
                                        },
                                        despawn_anim: DespawnAnimation::new(translation)
                                            .with_extra_progress(CLAW_ANIM_DURATAION),
                                    });
                                    commands.add_trauma(0.5);
                                }
                            }
                        }

                        println!("Successfully used item: {}", item.name);

                        if item.item_count > 0 {
                            // Set back item if it is not used up yet.
                            inventory.set(index, item);
                        }
                    }
                }
            }
            SelectionEvent::Deselected(entity) => *prev_selection = Some(*entity),
        }
    }
}
