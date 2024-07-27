use crate::screen::playing::GameState;
use crate::screen::Screen;
use bevy::prelude::*;
use sickle_ui::prelude::*;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, show_states)
            .add_systems(Update, update_states);
    }
}

#[derive(Component)]
pub struct GameStateLabel;
#[derive(Component)]
pub struct ScreenLabel;

pub fn show_states(mut commands: Commands) {
    commands.ui_builder(UiRoot).column(|ui| {
        ui.row(|ui| {
            ui.style().z_index(ZIndex::Global(10000));

            ui.column(|ui| {
                ui.label(LabelConfig::from("AppState")).insert(ScreenLabel);
                ui.label(LabelConfig::from("GameState"))
                    .insert(GameStateLabel);
            });
        });
    });
}

pub fn update_states(
    screen: Res<State<Screen>>,
    gamestate: Res<State<GameState>>,
    mut q_screen_label: Query<&mut Text, (With<ScreenLabel>, Without<GameStateLabel>)>,
    mut q_game_state_label: Query<&mut Text, (Without<ScreenLabel>, With<GameStateLabel>)>,
) {
    if screen.is_changed() {
        for mut t in q_screen_label.iter_mut() {
            t.sections[0] = TextSection::from(format!("{:?}", screen.get()));
        }
    }
    if gamestate.is_changed() {
        for mut t in q_game_state_label.iter_mut() {
            t.sections[0] = TextSection::from(format!("{:?}", gamestate.get()));
        }
    }
}
