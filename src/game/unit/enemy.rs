use bevy::{math::uvec2, prelude::*};

use crate::{
    game::{
        cycle::{EndTurn, Season, TimeOfDay},
        level::Terrain,
        map::{VillageMap, ROOK_MOVES},
        tile_set::{tile_coord_translation, TileSet, TILE_ANCHOR},
        unit::{spawn::SpawnAnimation, EnemyUnit, IsAirborne, UnitBundle},
    },
    screen::{playing::GameState, Screen},
};

use super::Movement;

/// Distance from border that the enemy will spawn in.
pub const ENEMY_SPAWN_RANGE: u32 = 2;
const SPAWN_TRIAL: usize = 10;

pub struct EnemyUnitPlugin;

impl Plugin for EnemyUnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(TimeOfDay::Night), spawn_enemies)
            .add_systems(
                Update,
                (enemies_path, move_enemies)
                    .run_if(in_state(Screen::Playing).and_then(in_state(GameState::EnemyTurn))),
            );
    }
}

fn enemies_path(
    mut commands: Commands,
    q_terrains: Query<&Terrain>,
    mut q_enemy_units: Query<(Entity, &Movement, Option<&IsAirborne>), With<EnemyUnit>>,
    mut end_turn_evt: EventReader<EndTurn>,
    mut village_map: ResMut<VillageMap>,
) {
    if end_turn_evt.is_empty() == false {
        end_turn_evt.clear();
        for (entity, movement, airborne) in q_enemy_units.iter_mut() {
            let Some(tile_coord) = village_map.object.locate(entity) else {
                continue;
            };

            let is_airborne = airborne.is_some();
            let Some(best_tile) = village_map.get_best_tile(
                tile_coord,
                movement.0,
                &ROOK_MOVES,
                is_airborne,
                &q_terrains,
            ) else {
                continue;
            };

            let Some((path, _)) = village_map.pathfind(
                &tile_coord,
                &best_tile,
                &ROOK_MOVES,
                is_airborne,
                &q_terrains,
            ) else {
                continue;
            };

            commands.entity(entity).insert(TilePath::new(path));
            village_map.object.remove(tile_coord);
            village_map.object.set(best_tile, entity);
        }
    }
}

fn move_enemies(
    mut commands: Commands,
    mut q_enemy_units: Query<(Entity, &mut Transform, &mut TilePath), With<EnemyUnit>>,
    time: Res<Time>,
) {
    const SPEED: f32 = 4.0;
    let Some((entity, mut transform, mut path)) = q_enemy_units.iter_mut().next() else {
        return;
    };

    // Prevent out of bounds overflow
    if path.index >= path.path.len() - 1 {
        error!("Attempted to retrieve out of bounds path...");
        commands.entity(entity).remove::<TilePath>();
        return;
    }

    let current_tile = path.path[path.index].as_vec2();
    let target_tile = path.path[path.index + 1].as_vec2();

    let diff = target_tile - current_tile;
    let length = diff.length();
    if length < f32::EPSILON {
        error!("Distance between target path and current path is too small!");
        commands.entity(entity).remove::<TilePath>();
        return;
    }
    // Normalize direction
    let norm_dir = diff / length;

    let travel_dist = SPEED * time.delta_seconds();
    path.factor = f32::min(path.factor + travel_dist / length, 1.0);

    let tile_coord = current_tile + norm_dir * path.factor * length;
    transform.translation = tile_coord_translation(tile_coord.x, tile_coord.y, 2.0);

    if path.factor >= 1.0 {
        if path.index >= path.path.len() - 2 {
            // No more paths left
            commands.entity(entity).remove::<TilePath>();
        } else {
            // Increment the index to move towards the next path
            path.index += 1;
            path.factor = 0.;
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    q_terrains: Query<&Terrain>,
    mut village_map: ResMut<VillageMap>,
    season: Res<Season>,
    tile_set: Res<TileSet>,
) {
    debug_assert!(
        village_map.size.x == village_map.size.y,
        "Map is not square."
    );
    let width = village_map.size.x;

    if ENEMY_SPAWN_RANGE * 2 > width {
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
        let mut tile_coord = IVec2::ZERO;
        for _ in 0..SPAWN_TRIAL {
            tile_coord = random_border_tile_coord(width, ENEMY_SPAWN_RANGE).as_ivec2();

            // There is something blocking the spawning location
            if village_map.object.get(tile_coord).is_some() {
                continue;
            }

            let Some(terrain) = village_map
                .ground
                .get(tile_coord)
                .and_then(|e| q_terrains.get(e).ok())
            else {
                error!("Unable to get terrain from tile coordinate: {tile_coord:?}");
                return;
            };

            match terrain {
                // Airborne enemy can be on top of anything water
                Terrain::Water if enemy.is_airborne == false => continue,
                _ => break,
            }
        }

        let translation = tile_coord_translation(tile_coord.x as f32, tile_coord.y as f32, 2.0);
        let mut enemy_entity = commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    anchor: TILE_ANCHOR,
                    ..default()
                },
                texture: tile_set.get(enemy.name),
                ..default()
            },
            UnitBundle::<EnemyUnit>::new(enemy.name)
                .with_hit_points(enemy.hit_points)
                .with_health(enemy.hit_points)
                .with_movement(enemy.movement),
            SpawnAnimation::new(translation),
            StateScoped(Screen::Playing),
        ));
        if enemy.is_airborne {
            enemy_entity.insert(IsAirborne);
        }
        village_map.object.set(tile_coord, enemy_entity.id());

        if let Some(best_tile) = village_map.get_best_tile(
            tile_coord,
            enemy.movement,
            &ROOK_MOVES,
            enemy.is_airborne,
            &q_terrains,
        ) {
            println!("best tile: {:?}", best_tile);
            println!(
                "{:?}",
                village_map.pathfind(
                    &tile_coord,
                    &best_tile,
                    &ROOK_MOVES,
                    enemy.is_airborne,
                    &q_terrains,
                )
            );
        }
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
}

impl EnemySpawn {
    pub const WEREWOLF: Self = Self {
        name: "werewolf",
        hit_points: 3,
        movement: 3,
        is_airborne: false,
    };
    pub const SLIME: Self = Self {
        name: "slime",
        hit_points: 4,
        movement: 2,
        is_airborne: false,
    };
    pub const BAT: Self = Self {
        name: "bat",
        hit_points: 2,
        movement: 4,
        is_airborne: true,
    };
}

#[derive(Component, Default, Debug, Clone)]
pub struct TilePath {
    pub path: Vec<IVec2>,
    /// Current path index that the entity is located at.
    pub index: usize,
    /// Animation factor between 2 tiles.
    pub factor: f32,
}

impl TilePath {
    pub fn new(path: Vec<IVec2>) -> Self {
        Self { path, ..default() }
    }
}
