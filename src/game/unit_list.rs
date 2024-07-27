use std::f32::MAX_10_EXP;

use bevy::color::palettes::css;
use bevy::prelude::*;
use sickle_ui::prelude::*;

use crate::ui::prelude::InteractionPalette;

use super::selection::SelectedTiles;
use super::selection::SelectedUnit;
use super::unit;
use super::unit::player::MAX_PLAYER_UNITS;
use super::unit::PlayerUnit;
use super::unit::UnitName;
use super::INVENTORY_CAPACITY;

#[derive(Component)]
pub struct UnitListContainer;

#[derive(Component)]
pub struct PlayerUnitIcon;

pub fn unit_list_layout(ui: &mut UiBuilder<Entity>) {
    ui.row(|ui| {
        ui.column(|ui| {
            ui.insert(UnitListContainer)
                .style()
                .align_items(AlignItems::Start)
                .justify_content(JustifyContent::Start)
                .background_color(Color::BLACK)
                .border_color(Color::WHITE)
                .border(UiRect::all(Val::Px(2.)))
                .padding(UiRect::all(Val::Px(2.)))
                .row_gap(Val::Px(2.));
        });
    });
}

#[derive(Resource, Default, Clone)]
pub struct PlayerUnitList(pub Vec<Entity>);

#[derive(Component)]
pub struct SelectPlayerUnitButton(pub Entity);

pub fn update_unit_list_container(
    mut local: Local<Vec<Entity>>,
    mut commands: Commands,
    container_query: Query<Entity, With<UnitListContainer>>,
    unit_query: Query<&UnitName>,
    player_unit_list: Res<PlayerUnitList>,
    selected_unit: Res<SelectedUnit>,
) {
    if *local != player_unit_list.0 {
        *local = player_unit_list.0.clone();
    } else if !selected_unit.is_changed() {
        return;
    }
    for entity in container_query.iter() {
        commands.entity(entity).despawn_descendants();
        let mut ui = commands.ui_builder(entity);
        for unit_entity in player_unit_list.0.iter() {
            let Ok(name) = unit_query.get(*unit_entity) else {
                continue;
            };
            let (text_color, back_color) = if selected_unit.entity == Some(*unit_entity) {
                (Color::BLACK, Color::WHITE)
            } else {
                (Color::WHITE, Color::BLACK)
            };
            ui.container(ButtonBundle { ..default() }, |ui| {
                ui.style()
                    .border_color(Color::WHITE)
                    .background_color(back_color)
                    .border(UiRect::all(Val::Px(2.)))
                    .column_gap(Val::Px(4.))
                    .width(Val::Percent(100.))
                    .padding(UiRect::all(Val::Px(4.)));
                ui.style()
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::SpaceBetween);

                ui.icon("tiles/human.png")
                    .insert(PlayerUnitIcon)
                    .style()
                    .width(Val::Px(64.))
                    .height(Val::Px(64.));

                ui.row(|ui| {
                    ui.style().padding(UiRect::all(Val::Px(4.)));
                    ui.label(LabelConfig::from(
                        name.0.split_whitespace().next().unwrap_or(""),
                    ))
                    .style()
                    .font_size(20.)
                    .font_color(text_color);
                });
            })
            .insert((
                InteractionPalette {
                    none: back_color,
                    hovered: css::DARK_RED.into(),
                    pressed: css::INDIAN_RED.into(),
                },
                SelectPlayerUnitButton(*unit_entity),
            ));
        }
    }
}

pub fn inventory_list_layout(ui: &mut UiBuilder<Entity>) {
    ui.column(|ui| {
        ui.style().align_items(AlignItems::End);
        ui.row(|ui| {
            ui.style().column_gap(Val::Px(2.));
            for _ in 0..INVENTORY_CAPACITY {
                ui.container(ButtonBundle { ..default() }, |ui| {
                    ui.icon("icons/population.png")
                        .style()
                        .width(Val::Px(48.))
                        .height(Val::Px(48.));
                })
                .insert((InteractionPalette {
                    none: css::BLACK.into(),
                    hovered: css::DARK_GRAY.into(),
                    pressed: css::WHITE.into(),
                },))
                .style()
                .padding(UiRect::all(Val::Px(10.0)))
                .border_radius(BorderRadius::all(Val::Px(5.0)));
            }
        });
    });
}

pub fn select_player_unit_btn_interaction(
    q_interactions: Query<(&Interaction, &SelectPlayerUnitButton), Changed<Interaction>>,
    mut selected_unit: ResMut<SelectedUnit>,
) {
    for (interaction, select) in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            selected_unit.entity = Some(select.0);
        }
    }
}
