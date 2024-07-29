use bevy::prelude::*;
use rand::Rng;

use crate::screen::Screen;

use super::components::Income;
use super::components::PopulationCapacity;
use super::cycle::TimeOfDay;
use super::cycle::Turn;
use super::resources::VillageEmployment;
use super::resources::VillageGold;
use super::resources::VillagePopulation;
use super::unit::Structure;
use super::update_resource_label;

pub struct EconomyPlugin;

impl Plugin for EconomyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VillageGold>()
            .init_resource::<VillagePopulation>()
            .add_systems(
                Update,
                (
                    update_resource_label::<VillageGold>(),
                    update_resource_label::<VillagePopulation>(),
                    update_resource_label::<VillageEmployment>(),
                )
                    .run_if(in_state(Screen::Playing)),
            )
            .add_systems(
                OnEnter(TimeOfDay::Day),
                update_income.run_if(|turn: Res<Turn>| turn.0 != 0),
            );
    }
}

pub fn update_income(
    mut population: ResMut<VillagePopulation>,
    mut gold: ResMut<VillageGold>,
    income_query: Query<&Income, With<Structure>>,
    cap_query: Query<&PopulationCapacity, With<Structure>>,
) {
    let mut total_population_capacity = 0;
    for population_capacity in cap_query.iter() {
        total_population_capacity += population_capacity.0;
    }

    gold.0 += population.0.min(total_population_capacity);

    for income in income_query.iter() {
        gold.0 += income.0;
    }
    let mut rng = rand::thread_rng();
    population.0 += 5 + rng.gen_range(0..10);
    population.0 = population.0.min(total_population_capacity);
}
