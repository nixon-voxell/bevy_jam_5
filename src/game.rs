//! Game mechanics and content.

use std::marker::PhantomData;

use bevy::{ecs::schedule::SystemConfigs, prelude::*};

pub mod assets;
pub mod audio;
pub mod components;
pub mod constants;
pub mod construction;
pub mod cycle;
pub mod deployment;
pub mod economy;
pub mod events;
pub mod inventory;
pub mod level;
pub mod map;
mod picking;
pub mod resources;
pub mod selection;
pub mod systems;
pub mod tile_set;
pub mod unit;
pub mod unit_list;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        audio::plugin,
        assets::plugin,
        cycle::CyclePlugin,
        tile_set::TileSetPlugin,
        level::LevelPlugin,
        economy::EconomyPlugin,
        selection::SelectionPlugin,
        picking::PickingPlugin,
        unit::UnitPlugin,
    ));
}

#[derive(Component)]
pub struct WatchRes<R: Resource> {
    phantom: PhantomData<R>,
}

impl<R: Resource> Default for WatchRes<R> {
    fn default() -> Self {
        WatchRes {
            phantom: PhantomData,
        }
    }
}

pub fn update_resource_label<R: Resource + ToString>() -> SystemConfigs {
    set_resource_label::<R>.run_if(resource_changed::<R>)
}

fn update_resource_label_system<R: Resource>(system: SystemConfigs) -> SystemConfigs {
    system.run_if(resource_changed::<R>)
}

fn set_resource_label<R: Resource + ToString>(
    mut q_texts: Query<&mut Text, With<WatchRes<R>>>,
    value: Res<R>,
) {
    for mut text in q_texts.iter_mut() {
        text.sections[0].value = value.to_string();
    }
}

pub const INVENTORY_CAPACITY: usize = 5;
pub const MERCHANT_ITEMS: usize = 3;

pub const MODAL_Z_LAYER: i32 = 100;
