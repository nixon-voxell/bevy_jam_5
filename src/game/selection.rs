use bevy::math::vec2;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::utils::HashSet;

use crate::game::map::ROOK_MOVES;

use super::picking::PickedTile;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedTiles>()
            .init_resource::<SelectionMap>()
            .add_systems(
                PostUpdate,
                (
                    show_selected_tiles.run_if(resource_changed::<SelectedTiles>),
                    add_selection,
                ),
            );
    }
}

/// Current selected unit, can be Player controlled, enemy or a building
#[derive(Resource, Default)]
pub struct SelectedUnit {
    pub entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct SelectedTiles {
    pub color: Color,
    pub tiles: HashSet<IVec2>,
}

#[derive(Resource, Default)]
pub struct SelectionMap {
    pub tiles: HashMap<IVec2, [Entity; 4]>,
}

#[derive(Component, Copy, Clone, Debug)]
pub enum SelectionEdge {
    North,
    East,
    South,
    West,
}

impl SelectionEdge {
    pub const ALL: [Self; 4] = [Self::North, Self::East, Self::South, Self::West];

    pub fn get_scalar(&self) -> Vec2 {
        match self {
            SelectionEdge::North => Vec2::ONE,
            SelectionEdge::East => vec2(1., -1.),
            SelectionEdge::South => -Vec2::ONE,
            SelectionEdge::West => vec2(-1., 1.),
        }
    }
}

pub fn show_selected_tiles(
    selected_tiles: Res<SelectedTiles>,
    tile_ids: Res<SelectionMap>,
    mut query: Query<(&mut Sprite, &mut Visibility), With<SelectionEdge>>,
) {
    for (_, mut vis) in query.iter_mut() {
        vis.set_if_neq(Visibility::Hidden);
    }

    for &tile in selected_tiles.tiles.iter() {
        let Some(s) = tile_ids.tiles.get(&tile) else {
            continue;
        };
        let neighbours = ROOK_MOVES
            .map(|m| tile + m)
            .map(|n| selected_tiles.tiles.contains(&n));
        for (i, a) in neighbours.into_iter().enumerate() {
            if !a {
                if let Ok((mut sprite, mut vis)) = query.get_mut(s[i]) {
                    sprite.color = selected_tiles.color;
                    *vis = Visibility::Visible;
                }
            }
        }
    }
}

pub fn add_selection(mut selected_tiles: ResMut<SelectedTiles>, picked_tile: Res<PickedTile>) {
    if let Some(picked_tile) = picked_tile.0 {
        selected_tiles.tiles.insert(picked_tile);
    }
}
