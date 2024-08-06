use super::actors::stats::Health;
use super::actors::stats::Movement;
use super::map::VillageMap;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

/// System param for accessing game data
#[derive(SystemParam)]
pub struct Game<'w, 's> {
    map: ResMut<'w, VillageMap>,
    health: Query<'w, 's, &'static mut Health>,
    movement: Query<'w, 's, &'static mut Movement>,
}
