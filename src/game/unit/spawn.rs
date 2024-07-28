use bevy::prelude::*;

use crate::screen::Screen;

/// Starting translation offset at spawn.
const SPAWN_START_OFFSET: Vec3 = Vec3::new(0.0, 300.0, 0.0);
/// Starting scale at spawn.
const SPAWN_START_SCALE: Vec3 = Vec3::splat(0.3);
/// Non-zero spawn animation duration.
const SPAWN_DURATION: f32 = 0.5;

pub struct SpawnUnitPlugin;

impl Plugin for SpawnUnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (spawn_animation, despawn_animation).run_if(in_state(Screen::Playing)),
        );
    }
}

fn spawn_animation(
    mut commands: Commands,
    mut q_transforms: Query<(Entity, &mut Transform, &mut Sprite, &mut SpawnAnimation)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut sprite, mut spawn) in q_transforms.iter_mut() {
        let mut factor = spawn.progress / SPAWN_DURATION;
        factor = f32::clamp(factor, 0.0, 1.0);
        factor = cubic::ease_in_out(factor);

        sprite.color.set_alpha(factor);
        transform.translation = Vec3::lerp(
            spawn.target_translation + SPAWN_START_OFFSET,
            spawn.target_translation,
            factor,
        );
        transform.scale = Vec3::lerp(SPAWN_START_SCALE, Vec3::ONE, factor);

        spawn.progress += time.delta_seconds();
        if spawn.progress > SPAWN_DURATION {
            commands.entity(entity).remove::<SpawnAnimation>();
        }
    }
}

fn despawn_animation(
    mut commands: Commands,
    mut q_transforms: Query<(Entity, &mut Transform, &mut Sprite, &mut DespawnAnimation)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut sprite, mut despawn) in q_transforms.iter_mut() {
        let mut factor = despawn.progress / SPAWN_DURATION;
        factor = f32::clamp(factor, 0.0, 1.0);
        factor = cubic::ease_in_out(factor);

        sprite.color.set_alpha(1.0 - factor);
        transform.translation = Vec3::lerp(
            despawn.origin_translation,
            despawn.origin_translation + SPAWN_START_OFFSET,
            factor,
        );
        transform.scale = Vec3::lerp(Vec3::ONE, SPAWN_START_SCALE, factor);

        despawn.progress += time.delta_seconds();
        if despawn.progress > SPAWN_DURATION {
            if despawn.recursive {
                commands.entity(entity).despawn_recursive();
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct DespawnAnimation {
    origin_translation: Vec3,
    progress: f32,
    recursive: bool,
}

impl DespawnAnimation {
    pub fn new(origin_translation: Vec3) -> Self {
        Self {
            origin_translation,
            progress: 0.0,
            recursive: false,
        }
    }

    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }
}

#[derive(Component, Clone, Copy)]
pub struct SpawnAnimation {
    target_translation: Vec3,
    progress: f32,
}

impl SpawnAnimation {
    pub fn new(target_translation: Vec3) -> Self {
        Self {
            target_translation,
            progress: 0.0,
        }
    }
}

pub mod cubic {
    #[inline]
    pub fn ease_in(t: f32) -> f32 {
        t * t * t
    }

    #[inline]
    pub fn ease_out(t: f32) -> f32 {
        let t = 1.0 - t;
        1.0 - t * t * t
    }

    #[inline]
    pub fn ease_in_out(t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            let t = 1.0 - t;
            1.0 - t * t * t * 4.0
        }
    }
}
