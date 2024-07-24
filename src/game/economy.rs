use std::marker::PhantomData;

use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;

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

fn update_resource_label<R: Resource + ToString>() -> SystemConfigs {
    set_resource_label::<R>.run_if(resource_changed::<R>)
}

fn set_resource_label<R: Resource + ToString>(
    mut q_texts: Query<&mut Text, With<WatchRes<R>>>, value: Res<R>
) {
    for mut text in q_texts.iter_mut() {
        text.sections[0].value = value.to_string();
    }
}


#[derive(Component, Default)]
pub struct WatchRes<R: Resource + ToString> {
    phantom: PhantomData<R>
}