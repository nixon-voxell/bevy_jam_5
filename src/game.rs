//! Game mechanics and content.

use bevy::prelude::*;

pub mod assets;
pub mod audio;
pub mod components;
pub mod cycle;
pub mod resources;
pub mod systems;
pub mod tile_map;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((cycle::CyclePlugin, audio::plugin, assets::plugin));
}
