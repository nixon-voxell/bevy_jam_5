use bevy::prelude::*;

use crate::game::cycle::EndTurn;

use crate::game::inventory::MaxInventorySize;

use crate::game::inventory::Inventory;

use crate::game::actors_list::PlayerActorList;
use crate::game::map::VillageMap;
pub use crate::game::picking::TilePressedEvent;
use crate::game::selection::SelectedActor;
use crate::game::tile_set::tile_coord_translation;
use crate::path_finding::tiles::TileDir;
use crate::screen::playing::GameState;

use super::*;

pub const INITIAL_PLAYER_UNITS: usize = 2;

pub fn spawn_player_unit(commands: &mut Commands, name: String) -> Entity {
    commands
        .spawn((
            SpatialBundle {
                visibility: Visibility::Hidden,
                ..default()
            },
            ActorBundle::<PlayerActor>::new(&name, TileDir::ALL.into())
                .with_health(3)
                .with_movement(3),
            MaxInventorySize(3),
            Inventory::default(),
        ))
        .id()
}

pub fn add_starting_player_units(
    mut available_names: ResMut<AvailableActorNames>,
    mut player_unit_list: ResMut<PlayerActorList>,
    mut commands: Commands,
) {
    player_unit_list.0.clear();
    for _ in 0..INITIAL_PLAYER_UNITS {
        let name = available_names.next_name();
        let id = spawn_player_unit(&mut commands, name);
        player_unit_list.0.push(id);
    }
}

pub fn move_unit(
    mut event_reader: EventReader<TilePressedEvent>,
    selected_unit: Res<SelectedActor>,
    mut village_map: ResMut<VillageMap>,
    mut turn_state_query: Query<
        (
            &mut ActorTurnState,
            &Movement,
            &mut Visibility,
            &mut Sprite,
            &mut Transform,
        ),
        With<PlayerActor>,
    >,
) {
    if let Some(TilePressedEvent(target)) = event_reader.read().last() {
        let Some(selected) = selected_unit.entity else {
            return;
        };

        let Ok((mut turn_state, movement, mut vis, mut sprite, mut transform)) =
            turn_state_query.get_mut(selected)
        else {
            return;
        };

        if turn_state.used_move || movement.0 == 0 {
            return;
        }

        let Some(current_pos) = village_map
            .actors
            .locate(selected)
            .filter(|pos| *pos != *target)
        else {
            return;
        };

        if village_map
            .flood(current_pos, movement.0, &TileDir::EDGES, false)
            .contains(target)
        {
            village_map.actors.set(*target, selected);
            turn_state.used_move = true;
            transform.translation =
                tile_coord_translation(target.x() as f32, target.y() as f32, 2.);
            transform.scale = Vec3::ONE;

            *vis = Visibility::Inherited;
            sprite.color.set_alpha(1.0);
        }
    }
}

pub fn reset_unit_turn_states(
    mut events: EventReader<EndTurn>,
    mut query: Query<&mut ActorTurnState>,
    state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for _ in events.read() {
        for mut turn_state in query.iter_mut() {
            turn_state.reset();
            if *state.get() == GameState::BattleTurn {
                next_game_state.set(GameState::EnemyTurn);
            }
        }
    }
}
