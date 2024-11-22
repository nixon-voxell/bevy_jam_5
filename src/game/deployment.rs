use crate::path_finding::tiles::Tile;
use crate::screen::Screen;

use rand::seq::SliceRandom;
use rand::thread_rng;

use super::actors_list::PlayerActorList;
use super::assets::SoundtrackKey;
use super::audio::soundtrack::PlaySoundtrack;
use super::map::VillageMap;
use super::picking::TilePressedEvent;
use super::selection::SelectedActor;
use super::selection::SelectedTiles;
use super::tile_set::tile_coord_translation;
use super::tile_set::TileSet;
use bevy::color::palettes::css::LIME;
use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum PlayerSprite {
    Human,
    Warrior,
    Orc,
    Viking,
}

impl PlayerSprite {
    pub fn texture_key(&self) -> &'static str {
        match self {
            PlayerSprite::Human => "human",
            PlayerSprite::Warrior => "warrior",
            PlayerSprite::Orc => "orc",
            PlayerSprite::Viking => "viking",
        }
    }
}

pub fn select_random_sprites() -> Vec<PlayerSprite> {
    let mut rng = thread_rng();
    let sprites = vec![
        PlayerSprite::Human,
        PlayerSprite::Warrior,
        PlayerSprite::Orc,
        PlayerSprite::Viking,
    ];

    // Shuffle and take two distinct sprites
    let mut shuffled_sprites = sprites.clone();
    shuffled_sprites.shuffle(&mut rng);

    shuffled_sprites.into_iter().take(2).collect()
}

pub fn deployment_setup(
    player_unit_list: Res<PlayerActorList>,
    mut selected_unit: ResMut<SelectedActor>,
    mut village_map: ResMut<VillageMap>,
    mut commands: Commands,
) {
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Battle));
    selected_unit.entity = player_unit_list.0.first().copied();
    let size = village_map.size;
    let r = IRect::from_corners(
        IVec2::ZERO,
        IVec2 {
            x: size.x() as i32,
            y: size.y() as i32,
        },
    )
    .inflate(-3);
    for x in r.min.x..r.max.x {
        for y in r.min.y..r.max.y {
            let value = Tile(x, y);
            village_map.deployment_zone.insert(value);
        }
    }
}

pub fn deployment_zone_visualization(
    village_map: Res<VillageMap>,
    mut selected_tiles: ResMut<SelectedTiles>,
) {
    selected_tiles
        .tiles
        .clone_from(&village_map.deployment_zone);
    selected_tiles.color = LIME.into();
}

pub fn is_deployment_ready(
    player_unit_list: Res<PlayerActorList>,
    village_map: Res<VillageMap>,
) -> bool {
    for entity in player_unit_list.0.iter() {
        if village_map.actors.locate(*entity).is_none() {
            return false;
        }
    }
    true
}

pub fn deploy_unit(
    mut events: EventReader<TilePressedEvent>,
    mut village_map: ResMut<VillageMap>,
    mut selected_unit: ResMut<SelectedActor>,
    player_unit_list: Res<PlayerActorList>,
    tile_set: Res<TileSet>,
    mut commands: Commands,
) {
    let Some(entity_to_deploy) = selected_unit.entity else {
        return;
    };

    // Get two random sprites
    let random_sprites = select_random_sprites();

    // Define sprite type for deployment based on random selection
    let sprite_type = match entity_to_deploy {
        entity if entity == player_unit_list.0[0] => random_sprites[0],
        entity if entity == player_unit_list.0[1] => random_sprites[1],
        _ => {
            println!("Unknown entity: {:?}", entity_to_deploy);
            return;
        }
    };

    if player_unit_list.0.contains(&entity_to_deploy) {
        if let Some(TilePressedEvent(target_tile)) = events.read().next() {
            if village_map.deployment_zone.contains(target_tile)
                && !village_map.actors.is_occupied(*target_tile)
            {
                let translation =
                    tile_coord_translation(target_tile.x() as f32, target_tile.y() as f32, 2.0);
                commands.entity(entity_to_deploy).insert((
                    SpriteBundle {
                        sprite: Sprite {
                            anchor: super::tile_set::TILE_ANCHOR,
                            ..default()
                        },
                        transform: Transform::from_translation(translation),
                        texture: tile_set.get(sprite_type.texture_key()),
                        ..default()
                    },
                    StateScoped(Screen::Playing),
                ));
                village_map.actors.set(*target_tile, entity_to_deploy);
                println!("Placing {} at {:?}", entity_to_deploy, target_tile);
                if let Some(next_unit) = player_unit_list
                    .0
                    .iter()
                    .find(|entity| village_map.actors.locate(**entity).is_none())
                {
                    println!("deployed: {entity_to_deploy:?}, next unit: {next_unit:?}");
                    selected_unit.set(*next_unit);
                }
            }
        }
    }

    events.clear();
}
