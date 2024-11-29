use crate::screen::playing::GameState;
use crate::screen::Screen;
use bevy::color::palettes::css::NAVY;
use bevy::color::palettes::css::RED;
use bevy::prelude::*;
use sickle_ui::prelude::*;
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, show_states)
            .add_systems(Update, (toggle_debug_panel, update_states));
    }
}

#[derive(Component)]
pub struct DebugPanelMarker;

#[derive(Component)]
pub struct GameStateLabel;
#[derive(Component)]
pub struct ScreenLabel;

pub fn show_states(mut commands: Commands) {
    commands.ui_builder(UiRoot).column(|ui| {
        ui.insert(DebugPanelMarker);
        ui.row(|ui| {
            ui.style()
                .margin(UiRect::all(Val::Percent(2.)))
                .z_index(ZIndex::Global(10000))
                .background_color(NAVY.into())
                .border(UiRect::all(Val::Px(2.)))
                .padding(UiRect::all(Val::Px(2.)))
                .border_color(RED.into());
            ui.column(|ui| {
                ui.label(LabelConfig::from("AppState"))
                    .insert(ScreenLabel)
                    .style()
                    .align_self(AlignSelf::Start);

                ui.label(LabelConfig::from("GameState"))
                    .insert(GameStateLabel)
                    .style()
                    .align_self(AlignSelf::Start);
            });
        });
    });
}

pub fn update_states(
    screen: Res<State<Screen>>,
    gamestate: Res<State<GameState>>,
    mut q_screen_label: Query<&mut Text, (With<ScreenLabel>, Without<GameStateLabel>)>,
    mut q_game_state_label: Query<&mut Text, (Without<ScreenLabel>, With<GameStateLabel>)>,
    asset_server: Res<AssetServer>,
) {
    if screen.is_changed() {
        for mut t in q_screen_label.iter_mut() {
            t.sections[0] = TextSection::new(
                format!("{:?}", screen.get()),
                TextStyle {
                    font: asset_server.load("fonts/GaramondLibre-Regular.otf"),
                    font_size: 20.,
                    color: Color::WHITE,
                },
            );
        }
    }
    if gamestate.is_changed() {
        for mut t in q_game_state_label.iter_mut() {
            t.sections[0] = TextSection::new(
                format!("{:?}", gamestate.get()),
                TextStyle {
                    font: asset_server.load("fonts/GaramondLibre-Regular.otf"),
                    font_size: 20.,
                    color: Color::WHITE,
                },
            );
        }
    }
}

pub fn toggle_debug_panel(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Style, With<DebugPanelMarker>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyD) && keyboard_input.pressed(KeyCode::ControlLeft) {
        for mut style in query.iter_mut() {
            style.display = match style.display {
                Display::None => Display::Flex,
                _ => Display::None,
            };
        }
    }
}
