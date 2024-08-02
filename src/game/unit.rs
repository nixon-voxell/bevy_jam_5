use crate::path_finding::tiles::Direction;
use crate::screen::Screen;
use crate::ui::icon_set::IconSet;
use bevy::prelude::*;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use self::enemy::EnemyUnitPlugin;
use self::spawn::{DespawnAnimation, SpawnAnimation, SpawnUnitPlugin};

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

const HIT_POINT_SIZE: Vec2 = Vec2::new(40.0, 40.0);
const HIT_POINT_GAP: f32 = 10.0;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnemyUnitPlugin, SpawnUnitPlugin))
            .add_systems(Update, health_ui.run_if(in_state(Screen::Playing)));
    }
}

fn health_ui(
    mut commands: Commands,
    mut q_hit_points: Query<
        (Entity, &MaxHealth, &Health, &mut HealthIcons, &Transform),
        Or<(Changed<MaxHealth>, Changed<Health>)>,
    >,
    q_is_player: Query<(), With<PlayerUnit>>,
    mut village_map: ResMut<VillageMap>,
    icon_set: Res<IconSet>,
) {
    for (entity, hit_point, health, mut icons, transform) in q_hit_points.iter_mut() {
        if health.0 == 0 {
            // Object dies
            let mut despawn_animation =
                DespawnAnimation::new(transform.translation).with_recursive(true);

            if q_is_player.contains(entity) {
                despawn_animation = despawn_animation.with_hide_only(true);
                // Player unit will only have 1 health for the next round
                commands.entity(entity).insert(Health(1));
            }
            commands.entity(entity).insert(despawn_animation);
            village_map.object.remove_entity(entity);
            return;
        }

        let is_icon_empty = icons.0.is_empty();
        // Remove previous health icons
        for icon in icons.0.iter() {
            commands.entity(*icon).despawn();
        }
        icons.0.clear();

        // Spawn health icons
        commands.entity(entity).with_children(|builder| {
            let hit_pointf = hit_point.0 as f32;
            let start_x = -HIT_POINT_SIZE.x * hit_pointf * 0.5;

            for index in 0..hit_point.0 {
                let indexf = index as f32;

                let color = match index < health.0 {
                    true => Srgba::RED,
                    false => Srgba::gray(0.2),
                };
                let translation = Vec3::new(
                    start_x + HIT_POINT_SIZE.x * indexf + HIT_POINT_GAP * indexf,
                    300.0,
                    100.0,
                );

                let mut icon = builder.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: color.into(),
                            custom_size: Some(HIT_POINT_SIZE),
                            ..default()
                        },
                        texture: icon_set.get("heart"),
                        transform: Transform::from_translation(translation),
                        ..default()
                    },
                    HealthIcon,
                ));

                if is_icon_empty {
                    icon.insert(SpawnAnimation::new(translation));
                }

                icons.0.push(icon.id());
            }
        });
    }
}

/// Amount of damage a unit can take before dying.
#[derive(Component, Copy, Clone, Debug, Deref, DerefMut, PartialEq)]
pub struct MaxHealth(pub u32);

/// Amount of health the unity current has.
#[derive(Component, Copy, Clone, Debug, Deref, DerefMut, PartialEq)]
pub struct Health(pub u32);

/// Health icon marker.
#[derive(Component)]
pub struct HealthIcon;

/// Vec of entities that holds the health icon sprites.
#[derive(Component, Default, Clone)]
pub struct HealthIcons(Vec<Entity>);

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
    pub hit_points: MaxHealth,
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
            hit_points: MaxHealth(2),
            health: Health(2),
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
    pub fn with_hit_points(mut self, hit_points: u32) -> Self {
        self.hit_points = MaxHealth(hit_points);
        self
    }

    pub fn with_health(mut self, health: u32) -> Self {
        self.health = Health(health);
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
    pub hit_points: MaxHealth,
    pub health: Health,
    pub health_icons: HealthIcons,
    pub structure: Structure,
    pub population_capacity: PopulationCapacity,
    pub layer_marker: ObjectTileLayer,
}

impl Default for StructureBundle {
    fn default() -> Self {
        Self {
            hit_points: MaxHealth(2),
            health: Health(2),
            health_icons: HealthIcons::default(),
            structure: Structure,
            layer_marker: ObjectTileLayer,
            population_capacity: PopulationCapacity(HOUSE_POPULATION_CAPACITY),
        }
    }
}
