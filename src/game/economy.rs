use bevy::prelude::*;

use crate::screen::playing::{GoldLabel, PopulationLabel};

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerGold>()
            .init_resource::<VillagePopulation>()
            .add_systems(
                Update,
                (
                    gold_label.run_if(resource_changed::<PlayerGold>),
                    population_label.run_if(resource_changed::<VillagePopulation>),
                ),
            );
    }
}

#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub struct PlayerGold(pub u32);

#[derive(Resource, Debug, Copy, Clone, PartialEq, Default)]
pub struct VillagePopulation(pub u32);

fn gold_label(mut q_texts: Query<&mut Text, With<GoldLabel>>, gold: Res<PlayerGold>) {
    let Ok(mut text) = q_texts.get_single_mut() else {
        return;
    };

    let section = &mut text.sections[0];
    section.value = gold.0.to_string();
}

fn population_label(
    mut q_texts: Query<&mut Text, With<PopulationLabel>>,
    population: Res<VillagePopulation>,
) {
    let Ok(mut text) = q_texts.get_single_mut() else {
        return;
    };

    let section = &mut text.sections[0];
    section.value = population.0.to_string();
}
