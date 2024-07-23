//! Spawn the main level by triggering other observers.

use bevy::prelude::*;

use crate::screen::Screen;

use self::level_asset::{LevelAssetPlugin, LevelMarker, Levels};

pub mod level_asset;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LevelAssetPlugin)
            .add_systems(Update, load_level.run_if(in_state(Screen::Playing)))
            .add_systems(OnEnter(Screen::Title), unload_all_level);
    }
}

fn load_level(mut q_visbilities: Query<&mut Visibility, With<LevelMarker>>, levels: Res<Levels>) {
    let Some(debug_level) = levels.0.get("debug_level") else {
        warn!("No debug level found..");
        return;
    };

    if let Some(mut vis) = debug_level
        .parent
        .and_then(|e| q_visbilities.get_mut(e).ok())
    {
        *vis = Visibility::Visible;
    }
}

fn unload_all_level(
    mut q_visbilities: Query<&mut Visibility, With<LevelMarker>>,
    levels: Res<Levels>,
) {
    for (name, level) in levels.0.iter() {
        info!("Unloading level: {name}");

        if let Some(mut vis) = level.parent.and_then(|e| q_visbilities.get_mut(e).ok()) {
            *vis = Visibility::Hidden;
        }
    }
}
