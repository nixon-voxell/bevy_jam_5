use bevy::color::palettes::css;
use bevy::prelude::*;
use sickle_ui::prelude::*;

use crate::ui::prelude::InteractionPalette;

use super::INVENTORY_CAPACITY;

pub fn unit_list_layout(ui: &mut UiBuilder<Entity>) {
    ui.row(|ui| {
        ui.column(|ui| {
            ui.style()
                .align_items(AlignItems::Start)
                .justify_content(JustifyContent::Start)
                .background_color(Color::BLACK)
                .border_color(Color::WHITE)
                .border(UiRect::all(Val::Px(2.)))
                .padding(UiRect::all(Val::Px(2.)))
                .row_gap(Val::Px(2.));

            for _ in 0..4 {
                ui.row(|ui| {
                    ui.style()
                        .border_color(Color::WHITE)
                        .border(UiRect::all(Val::Px(2.)))
                        .column_gap(Val::Px(4.))
                        .padding(UiRect::all(Val::Px(4.)));
                    ui.style()
                        .align_items(AlignItems::Center)
                        .justify_content(JustifyContent::SpaceBetween);

                    ui.icon("tiles/player_human.png")
                        .style()
                        .width(Val::Px(64.))
                        .height(Val::Px(64.));

                    ui.row(|ui| {
                        ui.style().padding(UiRect::all(Val::Px(4.)));
                        ui.label(LabelConfig::from("3 / 3")).style().font_size(20.);
                    });
                });
            }
        });
    });
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
