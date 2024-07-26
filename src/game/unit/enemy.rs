use bevy::{math::uvec2, prelude::*};

use crate::{
    game::{
        cycle::{Season, TimeOfDay},
        level::Terrain,
        map::{VillageMap, ROOK_MOVES},
        tile_set::{tile_coord_translation, TileSet, TILE_ANCHOR},
        unit::{EnemyUnit, UnitBundle},
    },
    screen::Screen,
};

/// Distance from border that the enemy will spawn in.
pub const ENEMY_SPAWN_RANGE: u32 = 2;
const SPAWN_TRIAL: usize = 10;

pub struct EnemyUnitPlugin;

impl Plugin for EnemyUnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(TimeOfDay::Night), spawn_enemies);
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
        let enemy_entity = commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    anchor: TILE_ANCHOR,
                    ..default()
                },
                transform: Transform::from_translation(translation),
                texture: tile_set.get(enemy.name),
                ..default()
            },
            UnitBundle::<EnemyUnit>::new(enemy.name)
                .with_hit_points(enemy.hit_points)
                .with_health(enemy.hit_points)
                .with_movement(enemy.movement),
            StateScoped(Screen::Playing),
        ));
        village_map.object.set(tile_coord, enemy_entity.id());

        println!(
            "{:?}",
            village_map.pathfind(
                &tile_coord,
                &IVec2::ZERO,
                &ROOK_MOVES,
                enemy.is_airborne,
                &q_terrains
            )
        )
    }

    // find_all_within_distance(, , )
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
