//! The screen state for the main game loop.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use sickle_ui::prelude::*;

use super::Screen;
use crate::game::{assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack};
use crate::ui::{palette::*, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnExit(Screen::Playing), exit_playing);

    app.add_systems(
        Update,
        return_to_title_screen
            .run_if(in_state(Screen::Playing).and_then(input_just_pressed(KeyCode::Escape))),
    );
}

fn enter_playing(mut commands: Commands) {
    // TODO: Initialize gameplay.
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
    commands
        .ui_builder(UiRoot)
        .column(|column| {
            column
                .style()
                .width(Val::Percent(100.0))
                .height(Val::Percent(100.0))
                .padding(UiRect::all(Val::Px(60.0)));

            // Top panel
            column.row(|row| {
                row.style()
                    .justify_content(JustifyContent::SpaceBetween)
                    .align_items(AlignItems::Center);

                row.label(LabelConfig::from("Season"))
                    .style()
                    .font_size(30.0);

                row.column(|_| {}).style().flex_grow(1.0);

                row.label(LabelConfig::from("Currency: "))
                    .style()
                    .font_size(20.0);

                row.label(LabelConfig::from("Population: "))
                    .style()
                    .font_size(20.0);

                row.column(|_| {}).style().width(Val::Px(20.0));

                row.container(ButtonBundle { ..default() }, |btn| {
                    btn.label(LabelConfig::from("Option"))
                        .style()
                        .font_size(20.0);
                })
                .insert(InteractionPalette {
                    none: NODE_BACKGROUND,
                    hovered: BUTTON_HOVERED_BACKGROUND,
                    pressed: BUTTON_PRESSED_BACKGROUND,
                })
                .style()
                .padding(UiRect::all(Val::Px(10.0)))
                .border_radius(BorderRadius::all(Val::Px(5.0)));
            });
            // Center panel
            column.row(|_row| {}).style().flex_grow(1.0);
            // Bottom panel
            column.row(|row| {
                row.label(LabelConfig::from("Turns until x: "))
                    .style()
                    .font_size(20.0);

                row.column(|_| {}).style().flex_grow(1.0);

                row.container(ButtonBundle { ..default() }, |btn| {
                    btn.label(LabelConfig::from("End Turn"))
                        .style()
                        .font_size(20.0);
                })
                .insert(InteractionPalette {
                    none: NODE_BACKGROUND,
                    hovered: BUTTON_HOVERED_BACKGROUND,
                    pressed: BUTTON_PRESSED_BACKGROUND,
                })
                .style()
                .padding(UiRect::all(Val::Px(10.0)))
                .border_radius(BorderRadius::all(Val::Px(5.0)));
            });
        })
        .insert(StateScoped(Screen::Playing));
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

fn return_to_title_screen(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
