use std::ops::{Index, IndexMut};

use bevy::prelude::*;
use bevy_enoki::prelude::*;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{AsRefStr, EnumCount, EnumIter};

pub(super) struct VfxPlugin;

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FireOneShotVfx>()
            .add_systems(Startup, setup_attack_vfx)
            .add_systems(Update, fire_oneshot_vfx);
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

fn setup_attack_vfx(mut commands: Commands, asset_server: Res<AssetServer>) {
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
