use bevy::prelude::*;

use crate::screen::Screen;

use super::update_resource_label;

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerGold>()
            .init_resource::<VillagePopulation>()
            .add_systems(
                Update,
                (
                    update_resource_label::<PlayerGold>(),
                    update_resource_label::<VillagePopulation>(),
                )
                    .run_if(in_state(Screen::Playing)),
            );
    }
}

#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub struct PlayerGold(pub u32);

impl std::fmt::Display for PlayerGold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub struct VillagePopulation(pub u32);

impl std::fmt::Display for VillagePopulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
