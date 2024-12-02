use bevy::color::palettes::css;
use bevy::math::uvec2;
use bevy::prelude::*;
use bevy_trauma_shake::TraumaCommands;

use crate::game::actors::spawn::SpawnAnimation;
use crate::game::actors::ActorBundle;
use crate::game::actors_list::PlayerActorList;
use crate::game::constants::*;
use crate::game::cycle::{DayCycle, Season, TimeOfDay, Turn};
use crate::game::level::Terrain;
use crate::game::map::VillageMap;
use crate::game::tile_set::{tile_coord_translation, TileSet, TILE_ANCHOR};
use crate::game::vfx::{FireOneShotVfx, OneShotVfx};
use crate::path_finding::tiles::{Tile, TileDir};
use crate::screen::playing::GameState;
use crate::screen::Screen;

use super::spawn::DespawnAnimation;
use super::{Directions, EnemyActor, Health, IsAirborne, Movement};

pub struct EnemyActorsPlugin;

impl Plugin for EnemyActorsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EnemyActionState>()
            .add_systems(OnEnter(TimeOfDay::Night), spawn_enemies)
            .add_systems(OnEnter(GameState::EnemyTurn), find_movement_path)
            .add_systems(
                Update,
                (
                    perform_attack.run_if(in_state(EnemyActionState::Attack)),
                    move_enemies
                        .run_if(in_state(EnemyActionState::Move))
                        .after(find_movement_path),
                )
                    .run_if(in_state(Screen::Playing).and_then(in_state(GameState::EnemyTurn))),
            );
    }
}

fn perform_attack(
    mut commands: Commands,
    mut q_enemy_attacks: Query<(Entity, &mut EnemyAttack), With<EnemyActor>>,
    q_not_enemy_units: Query<(), Without<EnemyActor>>,
    mut q_health: Query<&mut Health>,
    village_map: Res<VillageMap>,
    mut next_enemy_action_state: ResMut<NextState<EnemyActionState>>,
    time: Res<Time>,
    mut evw_oneshot_vfx: EventWriter<FireOneShotVfx>,
) {
    let Some((entity, mut enemy_attack)) = q_enemy_attacks.iter_mut().next() else {
        next_enemy_action_state.set(EnemyActionState::Move);
        return;
    };

    let tile = enemy_attack.tile.to_ivec2().as_vec2();
    let mut tile_trans = tile_coord_translation(tile.x, tile.y, 3.0);
    tile_trans.y += 100.0;

    if enemy_attack.factor == 0.0 {
        evw_oneshot_vfx.send(FireOneShotVfx(
            OneShotVfx::AttackFlash,
            Transform::from_translation(tile_trans),
        ));

        if village_map
            .actors
            .get(enemy_attack.tile)
            // Can only deal damage to non enemy units
            .filter(|e| q_not_enemy_units.contains(*e))
            .is_some_and(|e| q_health.contains(e))
        {
            evw_oneshot_vfx.send(FireOneShotVfx(
                OneShotVfx::BloodSplash,
                Transform::from_translation(tile_trans),
            ));
        }
        commands.add_trauma(0.5);
    }

    enemy_attack.factor += time.delta_seconds();
    if enemy_attack.factor >= ATK_ANIM_DURATION {
        // Deal damage
        if let Some(mut health) = village_map
            .actors
            .get(enemy_attack.tile)
            // Can only deal damage to non enemy units
            .filter(|e| q_not_enemy_units.contains(*e))
            .and_then(|e| q_health.get_mut(e).ok())
        {
            health.value = health.value.saturating_sub(1);
        }

        commands.entity(entity).remove::<EnemyAttack>();
    }
}

fn move_enemies(
    mut commands: Commands,
    mut q_enemy_units: Query<
        (Entity, &mut Transform, &Directions, Option<&mut TilePath>),
        With<EnemyActor>,
    >,
    q_not_enemy_units: Query<(), Without<EnemyActor>>,
    mut q_sprites: Query<(&mut Sprite, &mut Visibility)>,
    q_transforms: Query<&Transform, Without<EnemyActor>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_enemy_action_state: ResMut<NextState<EnemyActionState>>,
    mut village_map: ResMut<VillageMap>,
    player_unit_list: Res<PlayerActorList>,
    turn: Res<Turn>,
    time: Res<Time>,
    day_cycle: Res<DayCycle>,
) {
    if turn.0 != 0 && turn.0 % day_cycle.turns_per_day() == 0 {
        // Next day starts, clear all enemy units
        for (enemy_entity, transform, ..) in q_enemy_units.iter() {
            commands
                .entity(enemy_entity)
                .insert(DespawnAnimation::new(transform.translation).with_recursive(true));
            village_map.actors.remove_entity(enemy_entity);
        }

        // Hide all player units
        for &player_entity in player_unit_list.0.iter() {
            if village_map.actors.locate(player_entity).is_some() {
                if let Ok(transform) = q_transforms.get(player_entity) {
                    commands
                        .entity(player_entity)
                        .insert(DespawnAnimation::new(transform.translation).with_hide_only(true));
                }

                village_map.actors.remove_entity(player_entity);
            }
        }

        next_game_state.set(GameState::BuildingTurn);
        return;
    }

    let Some((entity, mut transform, directions, path)) =
        q_enemy_units.iter_mut().find(|(.., path)| path.is_some())
    else {
        next_enemy_action_state.set(EnemyActionState::Attack);
        println!("end enemy set BattleTurn");
        next_game_state.set(GameState::BattleTurn);
        return;
    };
    let mut path = path.unwrap();

    // No more paths left
    if path.index >= path.path.len() - 1 {
        commands.entity(entity).remove::<TilePath>();

        if let Some(enemy_tile) = path.path.last() {
            // Already in the best tile, find surroundings to attack!
            // for direction in enemy.
            for direction in directions.0.iter() {
                let attack_tile = enemy_tile.step(*direction);
                let Some(attack_entity) = village_map.actors.get(attack_tile) else {
                    continue;
                };

                if q_not_enemy_units.contains(attack_entity) {
                    // Mark tile for attack in the next enemy turn.
                    commands
                        .entity(entity)
                        .insert(EnemyAttack::new(attack_tile));

                    // Only 1 object can be attacked at the same time
                    break;
                }
            }
        }
        return;
    }

    let current_tile = path.path[path.index].to_ivec2().as_vec2();
    let target_tile = path.path[path.index + 1].to_ivec2().as_vec2();

    let diff = target_tile - current_tile;
    let length = diff.length();
    if length < f32::EPSILON {
        error!("Distance between target path and current path is too small!");
        commands.entity(entity).remove::<TilePath>();
        return;
    }
    // Normalize direction
    let norm_dir = diff / length;

    let travel_dist = ENEMY_MOVE_SPEED * time.delta_seconds();
    path.factor = f32::min(path.factor + travel_dist / length, 1.0);

    let tile_coord = current_tile + norm_dir * path.factor * length;
    transform.translation = tile_coord_translation(tile_coord.x, tile_coord.y, 2.0);

    if path.factor >= 1.0 {
        // Increment the index to move towards the next path
        path.index += 1;
        path.factor = 0.0;
    }
}

fn find_movement_path(
    mut commands: Commands,
    mut q_enemy_units: Query<
        (Entity, &Movement, &Directions, Option<&IsAirborne>),
        With<EnemyActor>,
    >,
    mut village_map: ResMut<VillageMap>,
) {
    // Regenerate heat map to check for player units as well.
    village_map.generate_heat_map(|e| q_enemy_units.contains(e));

    for (entity, movement, directions, airborne) in q_enemy_units.iter_mut() {
        let Some(enemy_tile) = village_map.actors.locate(entity) else {
            continue;
        };

        let is_airborne = airborne.is_some();
        let Some(best_tile) =
            village_map.get_best_tile(enemy_tile, movement.0, &directions.0, is_airborne)
        else {
            continue;
        };

        let Some((path, _)) =
            village_map.pathfind(&enemy_tile, &best_tile, &directions.0, is_airborne)
        else {
            continue;
        };

        commands.entity(entity).insert(TilePath::new(path));
        village_map.actors.remove(enemy_tile);
        village_map.actors.set(best_tile, entity);
    }
}

fn spawn_enemies(
    mut commands: Commands,
    mut village_map: ResMut<VillageMap>,
    season: Res<Season>,
    tile_set: Res<TileSet>,
) {
    debug_assert!(
        village_map.size.x() == village_map.size.y(),
        "Map is not square."
    );
    let width = village_map.size.x();

    if ENEMY_SPAWN_RANGE * 2 > width as u32 {
        warn!("Enemy spawn range ({ENEMY_SPAWN_RANGE} * 2) is larger than map size: {width}");
        return;
    }

    let enemies = match *season {
        Season::Summer => {
            // 2 werewolfs
            vec![EnemySpawn::WEREWOLF; 2]
        }
        Season::Autumn => {
            // 2 werewolfs, 1 slime
            vec![
                EnemySpawn::WEREWOLF,
                EnemySpawn::WEREWOLF,
                EnemySpawn::SLIME,
            ]
        }
        Season::Winter => {
            // 2 werewolfs, 1 slime, 2 bats
            vec![
                EnemySpawn::WEREWOLF,
                EnemySpawn::WEREWOLF,
                EnemySpawn::SLIME,
                EnemySpawn::BAT,
                EnemySpawn::BAT,
            ]
        }
    };

    for enemy in enemies {
        let mut tile_coord = Tile::ZERO;
        for _ in 0..SPAWN_TRIAL {
            tile_coord = random_border_tile_coord(width as u32, ENEMY_SPAWN_RANGE)
                .as_ivec2()
                .into();

            // There is something blocking the spawning location
            if village_map.actors.get(tile_coord).is_some() {
                continue;
            }

            let Some(terrain) = village_map.get_terrain(tile_coord) else {
                error!("Unable to get terrain from tile coordinate: {tile_coord:?}");
                return;
            };

            match terrain {
                // Airborne enemy can be on top of anything water
                Terrain::Water if enemy.is_airborne == false => continue,
                _ => break,
            }
        }

        let translation = tile_coord_translation(tile_coord.x() as f32, tile_coord.y() as f32, 2.0);
        let mut enemy_entity = commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    anchor: TILE_ANCHOR,
                    ..default()
                },
                texture: tile_set.get(enemy.name),
                ..default()
            },
            ActorBundle::<EnemyActor>::new(enemy.name, enemy.directions.to_vec())
                .with_health(enemy.hit_points)
                .with_health(enemy.hit_points)
                .with_movement(enemy.movement),
            SpawnAnimation::new(translation),
            StateScoped(Screen::Playing),
        ));
        if enemy.is_airborne {
            enemy_entity.insert(IsAirborne);
        }
        village_map.actors.set(tile_coord, enemy_entity.id());
    }
}

/// Get a random coordinate that is at the border of the grid.
pub fn random_border_tile_coord(width: u32, range: u32) -> UVec2 {
    let max_index = width - 1;
    let side = rand::random::<u32>() % 4;
    // |---------------| -> width
    //             |---| -> range
    // == == == == .. ..
    // == == == == .. ..
    let side_coord = uvec2(
        rand::random::<u32>() % (width - range),
        rand::random::<u32>() % range,
    );

    // Convert side coordinate into tile coordinate by performing 2d rotations based on side.
    match side {
        0 => side_coord,
        1 => uvec2(max_index - side_coord.y, side_coord.x),
        2 => uvec2(max_index - side_coord.x, max_index - side_coord.y),
        3 => uvec2(side_coord.y, max_index - side_coord.x),
        _ => unreachable!("Side should be within [0, 4)"),
    }
}

#[derive(Clone, Debug)]
pub struct EnemySpawn {
    pub name: &'static str,
    pub hit_points: u32,
    pub movement: u32,
    pub is_airborne: bool,
    pub directions: &'static [TileDir],
}

impl EnemySpawn {
    pub const WEREWOLF: Self = Self {
        name: "werewolf",
        hit_points: 3,
        movement: 3,
        is_airborne: false,
        directions: &TileDir::EDGES,
    };
    pub const SLIME: Self = Self {
        name: "slime",
        hit_points: 4,
        movement: 2,
        is_airborne: false,
        directions: &TileDir::EDGES,
    };
    pub const BAT: Self = Self {
        name: "bat",
        hit_points: 2,
        movement: 4,
        is_airborne: true,
        directions: &TileDir::ALL,
    };
}

#[derive(Component, Default, Debug, Clone)]
pub struct TilePath {
    pub path: Vec<Tile>,
    /// Current path index that the entity is located at.
    pub index: usize,
    /// Animation factor between 2 tiles.
    pub factor: f32,
}

impl TilePath {
    pub fn new(path: Vec<Tile>) -> Self {
        Self { path, ..default() }
    }
}

#[derive(Component, Default, Debug, Clone)]
pub struct EnemyAttack {
    pub tile: Tile,
    /// Animation factor.
    pub factor: f32,
}

impl EnemyAttack {
    pub fn new(tile: Tile) -> Self {
        Self { tile, ..default() }
    }
}

#[derive(Bundle)]
pub struct ClawMarkBundle {
    pub sprite: SpriteBundle,
    pub despawn_anim: DespawnAnimation,
}

#[derive(States, Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) enum EnemyActionState {
    #[default]
    Attack,
    Move,
}
