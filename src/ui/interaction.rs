use bevy::{math::VectorSpace, prelude::*};

use crate::game::{assets::SfxKey, audio::sfx::PlaySfx};

const PRESSED_SCALE: f32 = 0.9;
const HOVERED_SCALE: f32 = 1.1;
const ANIMATION_SPEED: f32 = 40.0;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InteractionPalette>();
    app.add_systems(
        Update,
        (
            apply_interaction_palette,
            trigger_interaction_sfx,
            apply_animation,
            run_scale_animation,
        ),
    );
}

pub type InteractionQuery<'w, 's, T> =
    Query<'w, 's, (&'static Interaction, T), Changed<Interaction>>;

/// Palette for widget interactions.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none: Color,
    pub hovered: Color,
    pub pressed: Color,
}

#[derive(Component)]
pub struct InteractAnimation(pub Interaction);

fn apply_interaction_palette(
    mut q_palettes: InteractionQuery<(&InteractionPalette, &mut BackgroundColor)>,
) {
    for (interaction, (palette, mut background)) in &mut q_palettes {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

fn apply_animation(mut commands: Commands, q_entities: InteractionQuery<Entity>) {
    for (interaction, entity) in q_entities.iter() {
        commands
            .entity(entity)
            .insert(InteractAnimation(*interaction));
    }
}

fn run_scale_animation(
    mut commands: Commands,
    mut q_animations: Query<(Entity, &mut Transform, &InteractAnimation)>,
    time: Res<Time>,
) {
    for (entity, mut transform, animation) in q_animations.iter_mut() {
        let scale = match animation.0 {
            Interaction::Hovered => HOVERED_SCALE,
            Interaction::Pressed => PRESSED_SCALE,
            Interaction::None => 1.0,
        };

        transform.scale = Vec3::lerp(
            transform.scale,
            Vec3::splat(scale),
            time.delta_seconds() * ANIMATION_SPEED,
        );

        if transform.scale == Vec3::ONE {
            commands.entity(entity).remove::<InteractAnimation>();
        }
    }
}

fn trigger_interaction_sfx(
    mut interactions: Query<&Interaction, Changed<Interaction>>,
    mut commands: Commands,
) {
    for interaction in &mut interactions {
        match interaction {
            Interaction::Hovered => commands.trigger(PlaySfx::Key(SfxKey::ButtonHover)),
            Interaction::Pressed => commands.trigger(PlaySfx::Key(SfxKey::ButtonPress)),
            _ => (),
        }
    }
}
