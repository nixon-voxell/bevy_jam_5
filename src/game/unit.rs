use bevy::prelude::*;

use crate::screen::Screen;

const HIT_POINT_SIZE: Vec2 = Vec2::new(40.0, 40.0);
const HIT_POINT_GAP: f32 = 10.0;

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, health_ui.run_if(in_state(Screen::Playing)));
    }
}

fn health_ui(
    mut commands: Commands,
    mut q_hit_points: Query<
        (Entity, &HitPoints, &Health, &mut HealthIcons),
        Or<(Changed<HitPoints>, Changed<Health>)>,
    >,
) {
    for (entity, hit_point, health, mut icons) in q_hit_points.iter_mut() {
        // Remove previous health icons
        for icon in icons.0.iter() {
            commands.entity(*icon).despawn();
        }
        icons.0.clear();

        // Spawn healht icons
        commands.entity(entity).with_children(|builder| {
            let hit_pointf = hit_point.0 as f32;
            let start_x = -HIT_POINT_SIZE.x * hit_pointf * 0.5;

            for index in 0..hit_point.0 {
                let indexf = index as f32;

                let color = match index < health.0 {
                    true => Srgba::RED,
                    false => Srgba::gray(0.2),
                };

                let icon_id = builder
                    .spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: color.into(),
                                custom_size: Some(HIT_POINT_SIZE),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(
                                start_x + HIT_POINT_SIZE.x * indexf + HIT_POINT_GAP * indexf,
                                300.0,
                                100.0,
                            )),
                            ..default()
                        },
                        HealthIcon,
                    ))
                    .id();

                icons.0.push(icon_id);
            }
        });
    }
}

/// Amount of damage a unit can take before dying.
#[derive(Component, Copy, Clone, Debug, Deref, DerefMut, PartialEq)]
pub struct HitPoints(pub u32);

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

#[derive(Bundle)]
pub struct UnitBundle<T: Component> {
    pub name: Name,
    pub hit_points: HitPoints,
    pub health: Health,
    pub health_icons: HealthIcons,
    pub movement: Movement,
    pub turn_state: UnitTurnState,
    pub unit: T,
    // pub abilities: Abilities,
}

impl<T: Component> UnitBundle<T>
where
    T: Default,
{
    pub fn new(name: &str) -> Self {
        Self {
            name: Name::new(String::from(name)),
            hit_points: HitPoints(2),
            health: Health(2),
            health_icons: HealthIcons::default(),
            movement: Movement(2),
            turn_state: UnitTurnState::default(),
            unit: T::default(),
        }
    }
}

impl<T: Component> UnitBundle<T> {
    pub fn with_hit_points(mut self, hit_points: u32) -> Self {
        self.hit_points = HitPoints(hit_points);
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

pub struct StructureBundle {
    pub name: Name,
    pub hit_points: HitPoints,
    pub health: Health,
    pub health_icons: HealthIcons,
    pub structure: Structure,
}

impl StructureBundle {
    pub fn new(name: &str) -> Self {
        Self {
            name: Name::new(String::from(name)),
            hit_points: HitPoints(2),
            health: Health(2),
            health_icons: HealthIcons::default(),
            structure: Structure,
        }
    }
}
