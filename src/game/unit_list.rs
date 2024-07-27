use std::f32::MAX_10_EXP;

use bevy::color::palettes::css;
use bevy::prelude::*;
use sickle_ui::prelude::*;

use crate::ui::prelude::InteractionPalette;

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

#[derive(Resource, Default)]
pub struct PlayerUnitList(pub Vec<Entity>);

pub fn update_unit_list_container(
    mut commands: Commands,
    container_query: Query<Entity, With<UnitListContainer>>,
    unit_query: Query<&UnitName>,
    player_unit_list: Res<PlayerUnitList>,
) {
    for entity in container_query.iter() {
        commands.entity(entity).despawn_descendants();
        let mut ui = commands.ui_builder(entity);
        for entity in player_unit_list.0.iter() {
            let Ok(name) = unit_query.get(*entity) else {
                continue;
            };
            ui.row(|ui| {
                ui.style()
                    .border_color(Color::WHITE)
                    .border(UiRect::all(Val::Px(2.)))
                    .column_gap(Val::Px(4.))
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
                    ui.label(LabelConfig::from(name.0.clone()))
                        .style()
                        .font_size(20.);
                });
            });
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
