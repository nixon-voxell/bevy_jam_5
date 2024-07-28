use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::utils::HashMap;
use sickle_ui::prelude::*;

use crate::screen::Screen;
use crate::ui::prelude::InteractionPalette;

use super::constants::BIG_TEXT_SIZE;
use super::constants::ICON_SIZE;
use super::constants::TEXT_SIZE;
use super::events::SelectStructureTypeEvent;
use super::resources::SelectedStructueType;

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StructureType {
    SmallHouse,
    House,
    BigHouse,
    Tavern,
    ArcherTower,
    Blacksmith,
}

impl StructureType {
    pub const ALL: [Self; 6] = [
        Self::SmallHouse,
        Self::House,
        Self::BigHouse,
        Self::Tavern,
        Self::ArcherTower,
        Self::Blacksmith,
    ];

    pub fn name(&self) -> &str {
        match self {
            StructureType::SmallHouse => "small house",
            StructureType::House => "house",
            StructureType::BigHouse => "big house",
            StructureType::Tavern => "tavern",
            StructureType::ArcherTower => "tower",
            StructureType::Blacksmith => "blacksmith",
        }
    }
}

#[derive(Component)]
pub struct BuildButton(pub StructureType);

pub struct StructureCost {
    pub turns: u32,
    pub workers: u32,
    pub gold: u32,
    /// only one can be built
    pub exclusive: bool,
}

#[derive(Resource, Deref)]
pub struct StructureCosts(pub HashMap<StructureType, StructureCost>);

impl Default for StructureCosts {
    fn default() -> Self {
        let costs: HashMap<_, _> = [
            (
                StructureType::SmallHouse,
                StructureCost {
                    turns: 2,
                    workers: 5,
                    gold: 25,
                    exclusive: false,
                },
            ),
            (
                StructureType::House,
                StructureCost {
                    turns: 4,
                    workers: 10,
                    gold: 50,
                    exclusive: false,
                },
            ),
            (
                StructureType::BigHouse,
                StructureCost {
                    turns: 6,
                    workers: 15,
                    gold: 100,
                    exclusive: false,
                },
            ),
            (
                StructureType::Tavern,
                StructureCost {
                    turns: 5,
                    workers: 10,
                    gold: 75,
                    exclusive: true,
                },
            ),
            (
                StructureType::ArcherTower,
                StructureCost {
                    turns: 3,
                    workers: 5,
                    gold: 25,
                    exclusive: false,
                },
            ),
            (
                StructureType::Blacksmith,
                StructureCost {
                    turns: 7,
                    workers: 10,
                    gold: 125,
                    exclusive: true,
                },
            ),
        ]
        .into_iter()
        .collect();

        Self(costs.into_iter().collect())
    }
}

pub fn building_panel_layout(mut commands: Commands, costs: Res<StructureCosts>) {
    commands.ui_builder(UiRoot).row(|ui| {
        ui.insert(StateScoped(Screen::Playing))
            .style()
            .width(Val::Percent(100.))
            .height(Val::Percent(100.))
            .justify_content(JustifyContent::End)
            .align_items(AlignItems::Center);
        ui.column(|ui| {
            ui.style()
                .height(Val::Auto)
                .padding(UiRect::all(Val::Px(5.)))
                .border(UiRect::all(Val::Px(2.)))
                .border_color(Color::WHITE)
                .border_radius(BorderRadius::all(Val::Px(5.)))
                .background_color(Color::BLACK)
                .row_gap(Val::Px(2.))
                .flex_grow(0.)
                .flex_shrink(0.)
                .margin(UiRect::all(Val::Px(10.)))
                .width(Val::Px(300.));

            ui.label(LabelConfig::from("Build"))
                .style()
                .font_size(BIG_TEXT_SIZE);

            for building_type in StructureType::ALL {
                let Some(cost) = costs.get(&building_type) else {
                    continue;
                };

                ui.container(ButtonBundle { ..default() }, |ui| {
                    ui.insert((
                        BuildButton(building_type),
                        InteractionPalette {
                            none: css::BLACK.into(),
                            hovered: css::TEAL.into(),
                            pressed: css::INDIAN_RED.into(),
                        },
                    ))
                    .style()
                    .border(UiRect::all(Val::Px(2.)))
                    .border_color(Color::WHITE)
                    .padding(UiRect::all(Val::Px(5.)));

                    ui.row(|ui| {
                        ui.style().justify_content(JustifyContent::SpaceBetween);
                        ui.label(LabelConfig::from(building_type.name()))
                            .style()
                            .font_size(TEXT_SIZE);

                        ui.column(|ui| {
                            ui.style().justify_content(JustifyContent::End);
                            ui.row(|ui| {
                                ui.style().column_gap(Val::Px(10.));
                                for (icon, value) in [
                                    ("icons/gold_coins.png", cost.gold.to_string()),
                                    ("icons/population.png", cost.workers.to_string()),
                                    ("icons/hourglass.png", cost.turns.to_string()),
                                ] {
                                    ui.row(|ui| {
                                        ui.style()
                                            .justify_content(JustifyContent::End)
                                            .column_gap(Val::Px(1.));
                                        ui.icon(icon).style().width(ICON_SIZE).height(ICON_SIZE);

                                        ui.label(LabelConfig::from(value))
                                            .style()
                                            .font_size(TEXT_SIZE);
                                    });
                                }
                            });
                        });
                    });
                });
            }
        });
    });
}

pub fn build_btn_interaction(
    q_interactions: Query<(&Interaction, &BuildButton), Changed<Interaction>>,
    mut build_select: EventWriter<SelectStructureTypeEvent>,
    mut selected_structure: ResMut<SelectedStructueType>,
) {
    for (i, b) in q_interactions.iter() {
        if *i == Interaction::Pressed {
            build_select.send(SelectStructureTypeEvent(b.0));
            selected_structure.0 = Some(b.0);
        }
    }
}

pub fn update_build_panel(
    selection: Res<SelectedStructueType>,
    mut q_interaction_pal: Query<(&BuildButton, &mut InteractionPalette, &mut BackgroundColor)>,
) {
    if let Some(s) = selection.0 {
        for (b, mut p, mut c) in q_interaction_pal.iter_mut() {
            if b.0 == s {
                p.none = css::RED.into();
            } else {
                p.none = Color::BLACK;
                c.0 = Color::BLACK;
            }
        }
    } else {
        for (_, mut p, mut c) in q_interaction_pal.iter_mut() {
            p.none = Color::BLACK;
            c.0 = Color::BLACK;
        }
    }
}
