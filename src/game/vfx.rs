use std::ops::{Index, IndexMut};

use bevy::prelude::*;
use bevy_enoki::prelude::*;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{AsRefStr, EnumCount, EnumIter};

use crate::{screen::Screen, ui::icon_set::IconSet};

use super::cycle::Season;

pub(super) struct VfxPlugin;

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FireOneShotVfx>()
            .add_systems(Startup, (setup_oneshot_vfx, setup_environment_vfx))
            .add_systems(
                Update,
                (
                    fire_oneshot_vfx,
                    (disable_environment_vfx, update_environment_vfx)
                        .chain()
                        .run_if(in_state(Screen::Playing).and_then(resource_changed::<Season>)),
                ),
            )
            .add_systems(OnEnter(Screen::Playing), update_environment_vfx)
            .add_systems(OnExit(Screen::Playing), disable_environment_vfx);
    }
}

fn fire_oneshot_vfx(
    mut q_states: Query<(&mut ParticleSpawnerState, &mut Transform)>,
    oneshot_vfx_map: Res<OneShotVfxMap>,
    mut evr_oneshot_vfx: EventReader<FireOneShotVfx>,
) {
    for oneshot in evr_oneshot_vfx.read() {
        let Ok((mut state, mut transform)) = q_states.get_mut(oneshot_vfx_map[oneshot.0]) else {
            continue;
        };

        state.active = true;
        *transform = oneshot.1;
    }
}

/// Enable the current [`Season`]'s vfx.
fn update_environment_vfx(
    mut q_states: Query<&mut ParticleSpawnerState>,
    env_vfx_map: Res<EnvironmentVfxMap>,
    season: Res<Season>,
) {
    let vfx = EnvironmentVfx::from_season(*season);
    if let Ok(mut state) = q_states.get_mut(env_vfx_map[vfx]) {
        state.active = true;
    }
}

/// Disable all environment vfx.
fn disable_environment_vfx(
    mut q_states: Query<&mut ParticleSpawnerState>,
    env_vfx_map: Res<EnvironmentVfxMap>,
) {
    for vfx in EnvironmentVfx::iter() {
        let Ok(mut state) = q_states.get_mut(env_vfx_map[vfx]) else {
            continue;
        };

        state.active = false;
    }
}

fn setup_oneshot_vfx(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sprite_mats: ResMut<Assets<SpriteParticle2dMaterial>>,
    icon_set: Res<IconSet>,
) {
    let map = OneShotVfxMap(
        OneShotVfx::iter()
            .map(|vfx| {
                commands
                    .spawn((
                        ParticleSpawnerBundle {
                            state: ParticleSpawnerState {
                                active: false,
                                ..default()
                            },
                            effect: asset_server.load(vfx.as_ref().to_owned() + ".ron"),
                            material: DEFAULT_MATERIAL,
                            ..default()
                        },
                        OneShot::Deactivate,
                    ))
                    .id()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    );

    commands
        .entity(map[OneShotVfx::AttackFlash])
        .remove::<Handle<ColorParticle2dMaterial>>()
        .insert(sprite_mats.add(SpriteParticle2dMaterial::new(
            icon_set.get("claw_mark"),
            1,
            1,
        )));

    commands.insert_resource(map);
}

fn setup_environment_vfx(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sprite_mats: ResMut<Assets<SpriteParticle2dMaterial>>,
    icon_set: Res<IconSet>,
) {
    let map = EnvironmentVfxMap(
        EnvironmentVfx::iter()
            .map(|vfx| {
                commands
                    .spawn((ParticleSpawnerBundle {
                        state: ParticleSpawnerState {
                            active: false,
                            ..default()
                        },
                        effect: asset_server.load(vfx.as_ref().to_owned() + ".ron"),
                        material: sprite_mats.add(SpriteParticle2dMaterial::new(
                            icon_set.get(vfx.texture_key()),
                            1,
                            1,
                        )),
                        transform: Transform::from_xyz(0.0, -600.0, 499.0),
                        ..default()
                    },))
                    .id()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    );

    commands.insert_resource(map);
}

#[derive(Resource, Deref)]
pub struct OneShotVfxMap([Entity; OneShotVfx::COUNT]);

impl Index<OneShotVfx> for OneShotVfxMap {
    type Output = Entity;

    fn index(&self, index: OneShotVfx) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<OneShotVfx> for OneShotVfxMap {
    fn index_mut(&mut self, index: OneShotVfx) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

#[derive(EnumCount, EnumIter, AsRefStr, Clone, Copy)]
#[strum(prefix = "enoki/")]
pub enum OneShotVfx {
    BloodSplash,
    AttackFlash,
}

#[derive(Event)]
pub struct FireOneShotVfx(pub OneShotVfx, pub Transform);

#[derive(Resource, Deref)]
pub struct EnvironmentVfxMap([Entity; EnvironmentVfx::COUNT]);

impl Index<EnvironmentVfx> for EnvironmentVfxMap {
    type Output = Entity;

    fn index(&self, index: EnvironmentVfx) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<EnvironmentVfx> for EnvironmentVfxMap {
    fn index_mut(&mut self, index: EnvironmentVfx) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

#[derive(EnumCount, EnumIter, AsRefStr, Clone, Copy)]
#[strum(prefix = "enoki/")]
pub enum EnvironmentVfx {
    Summer,
    Autumn,
    Winter,
}

impl EnvironmentVfx {
    pub fn texture_key(&self) -> &str {
        match self {
            EnvironmentVfx::Winter => "snowflake",
            _ => "leaf",
        }
    }

    pub fn from_season(season: Season) -> Self {
        match season {
            Season::Summer => Self::Summer,
            Season::Autumn => Self::Autumn,
            Season::Winter => Self::Winter,
        }
    }
}
