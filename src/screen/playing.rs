//! The screen state for the main game loop.

use bevy::color::palettes::css;
use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use sickle_ui::prelude::*;

use super::Screen;
use crate::game::cycle::EndTurn;
use crate::game::economy::{PlayerGold, VillagePopulation, WatchRes};
use crate::game::{assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack};
use crate::ui::{palette::*, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .add_systems(OnEnter(Screen::Playing), enter_playing)
        .add_systems(OnExit(Screen::Playing), exit_playing)
        .add_systems(OnEnter(GameState::Paused), enter_pause);

    app.add_systems(
        Update,
        (
            pause_btn_interaction,
            resume_btn_interaction,
            exit_btn_interaction,
            end_turn_btn_interaction,
        ),
    )
    .add_systems(
        Update,
        resume.run_if(in_state(GameState::Paused).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

/// Pause or resumed.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum GameState {
    #[default]
    Resumed,
    Paused,
}

#[derive(Component)]
pub struct ResumeButton;

#[derive(Component)]
pub struct PauseButton;

#[derive(Component)]
pub struct ExitButton;

#[derive(Component)]
pub struct EndTurnButton;

/// Label that shows the current [`Season`][Season].
///
/// [Season]: crate::game::cycle::Season
#[derive(Component)]
pub struct SeasonLabel;

/// Label that shows how many turns until [`TimeOfDay`][TimeOfDay].
///
/// [TimeOfDay]: crate::game::cycle::TimeOfDay
#[derive(Component)]
pub struct TurnUntilLabel;

fn economy_status_layout(ui: &mut UiBuilder<Entity>) {
    ui.column(|ui| {
        ui.style().justify_content(JustifyContent::Center);
        ui.row(|ui| {
            ui.style().column_gap(Val::Px(40.));
            ui.row(|ui| {
                ui.style().column_gap(Val::Px(4.));
                ui.icon("icons/gold-coins.png")
                    .style()
                    .width(Val::Px(32.0))
                    .height(Val::Px(32.0));

                ui.label(LabelConfig::from("0"))
                    .insert(WatchRes::<PlayerGold>::default())
                    .style()
                    .font_size(LABEL_SIZE);
            });

            ui.row(|ui| {
                ui.style().column_gap(Val::Px(4.));
                ui.icon("icons/population.png")
                    .style()
                    .width(Val::Px(32.0))
                    .height(Val::Px(32.0));

                ui.label(LabelConfig::from("0"))
                    .insert(WatchRes::<VillagePopulation>::default())
                    .style()
                    .font_size(LABEL_SIZE);
            });
        });
    });
}

fn enter_playing(mut commands: Commands) {
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
    commands
        .ui_builder(UiRoot)
        .column(|ui| {
            ui.style()
                .width(Val::Percent(100.0))
                .height(Val::Percent(100.0))
                .padding(UiRect::all(Val::Px(60.0)));

            // Top pane
            ui.row(|ui| {
                ui.style()
                    .justify_content(JustifyContent::SpaceBetween)
                    .align_items(AlignItems::Center)
                    .column_gap(Val::Px(40.));

                ui.label(LabelConfig::from("Season"))
                    .insert(SeasonLabel)
                    .style()
                    .font_size(HEADER_SIZE);

                ui.column(|_| {}).style().flex_grow(1.0);

                economy_status_layout(ui);

                ui.column(|_| {}).style().width(Val::Px(20.0));

                ui.container(ButtonBundle { ..default() }, |ui| {
                    ui.label(LabelConfig::from("Pause"))
                        .style()
                        .font_size(LABEL_SIZE);
                })
                .insert((
                    InteractionPalette {
                        none: NODE_BACKGROUND,
                        hovered: BUTTON_HOVERED_BACKGROUND,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    // Pause button component
                    PauseButton,
                ))
                .style()
                .padding(UiRect::all(Val::Px(10.0)))
                .border_radius(BorderRadius::all(Val::Px(5.0)));
            });
            // Center panel
            ui.row(|_ui| {}).style().flex_grow(1.0);
            // Bottom panel
            ui.row(|ui| {
                ui.label(LabelConfig::from("Turn Until"))
                    .insert(TurnUntilLabel)
                    .style()
                    .font_size(LABEL_SIZE);

                ui.column(|_| {}).style().flex_grow(1.0);

                ui.container(ButtonBundle { ..default() }, |ui| {
                    ui.label(LabelConfig::from("End Turn"))
                        .style()
                        .font_size(LABEL_SIZE);
                })
                .insert((
                    InteractionPalette {
                        none: css::RED.into(),
                        hovered: css::DARK_RED.into(),
                        pressed: css::INDIAN_RED.into(),
                    },
                    EndTurnButton,
                ))
                .style()
                .padding(UiRect::all(Val::Px(10.0)))
                .border_radius(BorderRadius::all(Val::Px(5.0)));
            });
        })
        .insert(StateScoped(Screen::Playing));
}

fn resume(mut next_game_state: ResMut<NextState<GameState>>) {
    next_game_state.set(GameState::Resumed);
}

fn enter_pause(mut commands: Commands) {
    commands
        .ui_builder(UiRoot)
        .column(|ui| {
            ui.style()
                .justify_content(JustifyContent::Center)
                .justify_items(JustifyItems::Center)
                .justify_self(JustifySelf::Center);

            ui.row(|ui| {
                ui.column(|ui| {
                    ui.label(LabelConfig::from("Paused"))
                        .style()
                        .font_size(40.0);

                    ui.column(|_| {}).style().height(Val::Px(20.0));

                    // Resume button
                    ui.container(ButtonBundle { ..default() }, |ui| {
                        ui.label(LabelConfig::from("Resume"))
                            .style()
                            .font_size(LABEL_SIZE);
                    })
                    .insert((
                        InteractionPalette {
                            none: css::GREEN.into(),
                            hovered: css::DARK_GREEN.into(),
                            pressed: css::GREEN.into(),
                        },
                        ResumeButton,
                    ))
                    .style()
                    .padding(UiRect::all(Val::Px(10.0)))
                    .border_radius(BorderRadius::all(Val::Px(5.0)));

                    ui.column(|_| {}).style().height(Val::Px(20.0));

                    // Exit button
                    ui.container(ButtonBundle { ..default() }, |ui| {
                        ui.label(LabelConfig::from("Exit"))
                            .style()
                            .font_size(LABEL_SIZE);
                    })
                    .insert((
                        InteractionPalette {
                            none: css::RED.into(),
                            hovered: css::DARK_RED.into(),
                            pressed: css::RED.into(),
                        },
                        ExitButton,
                    ))
                    .style()
                    .padding(UiRect::all(Val::Px(10.0)))
                    .border_radius(BorderRadius::all(Val::Px(5.0)));
                });
            })
            .style()
            .padding(UiRect::all(Val::Px(18.0)))
            .border_radius(BorderRadius::all(Val::Px(8.0)))
            .background_color(Color::srgba(0.0, 0.0, 0.0, 0.6));
        })
        .insert(StateScoped(GameState::Paused));
}

fn pause_btn_interaction(
    q_interactions: Query<&Interaction, (Changed<Interaction>, With<PauseButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for interaction in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            next_game_state.set(GameState::Paused);
        }
    }
}

fn resume_btn_interaction(
    q_interactions: Query<&Interaction, (Changed<Interaction>, With<ResumeButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for interaction in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            next_game_state.set(GameState::Resumed);
        }
    }
}

fn exit_btn_interaction(
    q_interactions: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    for interaction in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            // Resumed is default state that is needed
            // when we go back into the game later.
            next_game_state.set(GameState::Resumed);
            next_screen.set(Screen::Title);
        }
    }
}

fn end_turn_btn_interaction(
    q_interactions: Query<&Interaction, (Changed<Interaction>, With<EndTurnButton>)>,
    mut next_turn_evt: EventWriter<EndTurn>,
) {
    for interaction in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            next_turn_evt.send(EndTurn);
        }
    }
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}
