use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;

use crate::screen::playing::{GoldLabel, PopulationLabel, ResLabel};

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerGold>()
            .init_resource::<VillagePopulation>()
            .add_systems(
                Update,
                (
                    update_resource_label::<GoldLabel>(),
                    update_resource_label::<PopulationLabel>(),
                ),
            );
    }
}

#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub struct PlayerGold(pub u32);

impl ToString for PlayerGold {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub struct VillagePopulation(pub u32);

impl ToString for VillagePopulation {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

fn update_resource_label<T: ResLabel>() -> SystemConfigs {
    set_resource_label::<T>.run_if(resource_changed::<<T as ResLabel>::WatchedRes>)
}

fn set_resource_label<T: ResLabel>(
    mut q_texts: Query<&mut Text, With<T>>, value: Res<T::WatchedRes>
) {
    for mut text in q_texts.iter_mut() {
        text.sections[0].value = value.to_string();
    }
}
