//! Game mechanics and content.

use std::marker::PhantomData;

use bevy::{ecs::schedule::SystemConfigs, prelude::*};

use crate::tile_selection;

pub mod assets;
pub mod audio;
pub mod components;
pub mod cycle;
pub mod economy;
pub mod level;
pub mod map;
pub mod resources;
pub mod systems;
pub mod tile_set;
pub mod unit;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        audio::plugin,
        assets::plugin,
        cycle::CyclePlugin,
        tile_set::TileSetPlugin,
        level::LevelPlugin,
        economy::EconomyPlugin,
        tile_selection::TileSelectionPlugin,
        unit::UnitPlugin,
    ));
}

#[derive(Component, Default)]
pub struct WatchRes<R: Resource> {
    phantom: PhantomData<R>,
}

fn update_resource_label<R: Resource + ToString>() -> SystemConfigs {
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
