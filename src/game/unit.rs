use crate::path_finding::tiles::Direction;
use crate::screen::Screen;
use bevy::prelude::*;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use self::enemy::EnemyUnitPlugin;
use self::spawn::{DespawnAnimation, SpawnUnitPlugin};

use super::components::{ObjectTileLayer, PopulationCapacity};
use super::constants::HOUSE_POPULATION_CAPACITY;
use super::map::VillageMap;

pub mod enemy;
pub mod player;
pub mod spawn;

/// Character names generated from some random name generator
pub const NAMES: &[&str] = &[
    "Alaric Von Hohenberg",
    "Isolde De Sauveterre",
    "Dorian Blackwood",
    "Elara Valois",
    "Lucian Drakovich",
    "Seraphina Ravenscroft",
    "Thaddeus Greystone",
    "Morgana Devereux",
    "Victor Falkenrath",
    "Selene Montclair",
    "Tamachag Altan",
    "Xartsaga Borjigin",
    "Hychyt Chuluun",
    "Shilugei Baatar",
    "Khuchar Erdene",
    "Dodai Ganzorig",
    "Sibaguchu Oyun",
    "Adkiragh Sukhbaatar",
    "Jeder Temujin",
    "Gugun Munkhbat",
    "Hao Shuren",
    "Qiao Kang",
    "Dijewer de Weert",
    "Jacop Janssens",
    "Valentijn Hinckaert",
    "Valck Heyns",
    "Jeroom Michels",
    "Aeriaen van der Gracht",
    "Frederico de Nagele",
    "Egghel van Teijlingen",
    "Gabriel van der Molen",
    "Filips Schiffel",
    "Beco de Caria",
    "Jorge Mendanha",
    "Guomez de Monte Arroio",
    "Eytor d'Abrantes",
    "Olavi Paasio",
    "Alex Rautiainen",
    "Heikki Honkanen",
    "Lennart Soininen",
    "Eerik Ilves",
    "Bekir Burcak",
    "Akpolat Samdereli",
    "Erhan Calik",
    "Sariaslan Asena",
    "Toujou Dayu",
    "Wakuni Rikyu",
    "Yoshihisa Kimitada",
    "Chintan Haque",
    "Ashish Bhattacharya",
];

#[derive(Resource)]
pub struct AvailableUnitNames(pub Vec<&'static str>);

impl Default for AvailableUnitNames {
    fn default() -> Self {
        let mut names_vec: Vec<&'static str> = NAMES.to_vec();
        let mut rng = thread_rng();
        names_vec.shuffle(&mut rng);
        AvailableUnitNames(names_vec)
    }
}

impl AvailableUnitNames {
    pub fn next_name(&mut self) -> String {
        self.0
            .pop()
            .map(|name| name.to_string())
            .unwrap_or("Unnamed".to_string())
    }
}
pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnemyUnitPlugin, SpawnUnitPlugin))
            .add_systems(Update, health_ui.run_if(in_state(Screen::Playing)));
    }
}

fn health_ui(
    mut commands: Commands,
    mut q_hit_points: Query<(Entity, &Health, &Transform), Changed<Health>>,
    q_is_player: Query<(), With<PlayerUnit>>,
    mut village_map: ResMut<VillageMap>,
    // icon_set: Res<IconSet>,
) {
    for (entity, health, transform) in q_hit_points.iter_mut() {
        if health.value == 0 {
            // Object dies
            let mut despawn_animation =
                DespawnAnimation::new(transform.translation).with_recursive(true);

            if q_is_player.contains(entity) {
                despawn_animation = despawn_animation.with_hide_only(true);
                // Player unit will only have 1 health for the next round
                commands.entity(entity).insert(Health {
                    value: 1,
                    ..*health
                });
            }
            commands.entity(entity).insert(despawn_animation);
            village_map.object.remove_entity(entity);
            return;
        }
    }
}

/// Amount of health the unity current has.
#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct Health {
    pub value: u32,
    pub max: u32,
}

impl Health {
    pub fn new(value: u32) -> Self {
        Self { value, max: value }
    }

    pub fn is_full(self) -> bool {
        self.value == self.max
    }

    pub fn is_empty(self) -> bool {
        self.value == 0
    }

    pub fn heal(mut self, value: u32) -> u32 {
        self.value = (self.value + value).min(self.max);
        self.value
    }

    pub fn hurt(mut self, value: u32) -> u32 {
        self.value = self.value.saturating_sub(value);
        self.value
    }
}

/// Number of tiles a unit can move per turn.
#[derive(Component, Copy, Clone, Debug, Deref, DerefMut, PartialEq)]
pub struct Movement(pub u32);

/// Marker component for Player controlled units.
#[derive(Component, Default, Copy, Clone, Debug)]
pub struct PlayerUnit;

/// Marker component for Enemy units.
#[derive(Component, Default, Copy, Clone, Debug)]
pub struct EnemyUnit;

/// Marker component for airborne units.
#[derive(Component, Default, Copy, Clone, Debug)]
pub struct IsAirborne;

/// Directions a unit can move.
#[derive(Component, Default, Clone, Debug)]
pub struct Directions(pub Vec<Direction>);

/// Has unit moved or performed an action yet.
/// Needs to be reset to default after each turn (Not good?).
#[derive(Component, Default, Debug)]
pub struct UnitTurnState {
    pub used_move: bool,
    pub used_action: bool,
}

impl UnitTurnState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Component, Default, PartialEq, Debug)]
pub struct UnitName(pub String);

#[derive(Bundle)]
pub struct UnitBundle<T: Component> {
    pub name: UnitName,
    pub health: Health,
    pub health_icons: HealthIcons,
    pub movement: Movement,
    pub turn_state: UnitTurnState,
    pub unit: T,
    pub layer_marker: ObjectTileLayer,
    pub directions: Directions,
    // pub abilities: Abilities,
}

impl<T: Component> UnitBundle<T>
where
    T: Default,
{
    pub fn new(name: &str, directions: Vec<Direction>) -> Self {
        Self {
            name: UnitName(String::from(name)),
            health: Health::new(2),
            health_icons: HealthIcons::default(),
            movement: Movement(2),
            turn_state: UnitTurnState::default(),
            unit: T::default(),
            layer_marker: ObjectTileLayer,
            directions: Directions(directions),
        }
    }
}

impl<T: Component> UnitBundle<T> {
    pub fn with_health(mut self, value: u32) -> Self {
        self.health = Health::new(value);
        self
    }

    pub fn with_movement(mut self, movement: u32) -> Self {
        self.movement = Movement(movement);
        self
    }
}

/// Marker component for a building
#[derive(Component)]
pub struct Structure;

#[derive(Bundle)]
pub struct StructureBundle {
    pub health: Health,
    pub health_icons: HealthIcons,
    pub structure: Structure,
    pub population_capacity: PopulationCapacity,
    pub layer_marker: ObjectTileLayer,
}

impl Default for StructureBundle {
    fn default() -> Self {
        Self {
            health: Health::new(2),
            health_icons: HealthIcons::default(),
            structure: Structure,
            layer_marker: ObjectTileLayer,
            population_capacity: PopulationCapacity(HOUSE_POPULATION_CAPACITY),
        }
    }
}
