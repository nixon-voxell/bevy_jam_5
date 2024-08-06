//! Actors are entities that are placed in the actor layer of the village map
//! They represent any substantial game objects: player controlled characters, buildings, monsters, trees, etc
//! Each map tile can hold a maximum of one actor.

use crate::path_finding::tiles::Direction;
use crate::screen::Screen;
use bevy::prelude::*;
use enemy::EnemyActorsPlugin;
use rand::prelude::SliceRandom;
use rand::thread_rng;
use spawn::SpawnActorsPlugin;
use stats::{ActorName, Health, Movement};

use self::spawn::DespawnAnimation;

use super::components::{ActorTileLayer, PopulationCapacity};
use super::constants::HOUSE_POPULATION_CAPACITY;
use super::map::VillageMap;

pub mod enemy;
pub mod player;
pub mod spawn;
pub mod stats;

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
pub struct AvailableActorNames(pub Vec<&'static str>);

impl Default for AvailableActorNames {
    fn default() -> Self {
        let mut names_vec: Vec<&'static str> = NAMES.to_vec();
        let mut rng = thread_rng();
        names_vec.shuffle(&mut rng);
        AvailableActorNames(names_vec)
    }
}

impl AvailableActorNames {
    pub fn next_name(&mut self) -> String {
        self.0
            .pop()
            .map(|name| name.to_string())
            .unwrap_or("Unnamed".to_string())
    }
}
pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnemyActorsPlugin, SpawnActorsPlugin))
            .add_systems(Update, health_ui.run_if(in_state(Screen::Playing)));
    }
}

fn health_ui(
    mut commands: Commands,
    mut q_hit_points: Query<(Entity, &Health, &Transform), Changed<Health>>,
    q_is_player: Query<(), With<PlayerActor>>,
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
                // Player actor will only have 1 health for the next round
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

/// Marker component for Player controlled actors.
#[derive(Component, Default, Copy, Clone, Debug)]
pub struct PlayerActor;

/// Marker component for Enemy actors.
#[derive(Component, Default, Copy, Clone, Debug)]
pub struct EnemyActor;

/// Marker component for airborne actors.
#[derive(Component, Default, Copy, Clone, Debug)]
pub struct IsAirborne;

/// Directions a actor can move.
#[derive(Component, Default, Clone, Debug)]
pub struct Directions(pub Vec<Direction>);

/// Has actor moved or performed an action yet.
/// Needs to be reset to default after each turn (Not good?).
#[derive(Component, Default, Debug)]
pub struct ActorTurnState {
    pub used_move: bool,
    pub used_action: bool,
}

impl ActorTurnState {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Bundle)]
pub struct ActorBundle<T: Component> {
    pub name: ActorName,
    pub health: Health,
    pub movement: Movement,
    pub turn_state: ActorTurnState,
    pub actor: T,
    pub layer_marker: ActorTileLayer,
    pub directions: Directions,
    // pub abilities: Abilities,
}

impl<T: Component> ActorBundle<T>
where
    T: Default,
{
    pub fn new(name: &str, directions: Vec<Direction>) -> Self {
        Self {
            name: ActorName(String::from(name)),
            health: Health::new(2),
            movement: Movement(2),
            turn_state: ActorTurnState::default(),
            actor: T::default(),
            layer_marker: ActorTileLayer,
            directions: Directions(directions),
        }
    }
}

impl<T: Component> ActorBundle<T> {
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
    pub structure: Structure,
    pub population_capacity: PopulationCapacity,
    pub layer_marker: ActorTileLayer,
}

impl Default for StructureBundle {
    fn default() -> Self {
        Self {
            health: Health::new(2),
            structure: Structure,
            layer_marker: ActorTileLayer,
            population_capacity: PopulationCapacity(HOUSE_POPULATION_CAPACITY),
        }
    }
}
