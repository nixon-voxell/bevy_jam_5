use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::utils::HashMap;
use sickle_ui::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

use crate::screen::Screen;
use crate::ui::prelude::InteractionPalette;

use super::actors::spawn::SpawnAnimation;
use super::actors::Structure;
use super::actors::StructureBundle;
use super::components::ArcherTower;
use super::components::Blacksmith;
use super::components::BuildingProgressLabel;
use super::components::ConstructionWorkers;
use super::components::House;
use super::components::RemainingConstructionTurns;
use super::components::Tavern;
use super::constants::BIG_TEXT_SIZE;
use super::constants::ICON_SIZE;
use super::constants::TEXT_SIZE;
use super::events::EndDayTurn;
use super::events::SelectStructureTypeEvent;
use super::game_params::Game;
use super::map::VillageMap;
use super::picking::PickableTile;
use super::picking::TilePressedEvent;
use super::resources::SelectedStructueType;
use super::resources::VillageEmployment;
use super::resources::VillageGold;
use super::resources::VillagePopulation;
use super::tile_set::tile_coord_translation;
use super::tile_set::TileSet;
use super::tile_set::TILE_ANCHOR;

#[derive(Component, EnumIter, AsRefStr, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StructureType {
    House,
    Tavern,
    ArcherTower,
    Blacksmith,
}

impl StructureType {
    pub fn name(&self) -> &str {
        match self {
            StructureType::House => "House",
            StructureType::Tavern => "Tavern",
            StructureType::ArcherTower => "Tower",
            StructureType::Blacksmith => "Blacksmith",
        }
    }

    pub fn tile_texture(&self) -> &str {
        match self {
            StructureType::House => "house",
            StructureType::Tavern => "tavern",
            StructureType::ArcherTower => "tower",
            StructureType::Blacksmith => "blacksmith",
        }
    }
}

#[derive(Component)]
pub struct BuildButton(pub StructureType);

#[derive(Component)]
pub struct CancelBuildButton;

pub struct StructureCost {
    pub turns: u32,
    pub workers: u32,
    pub gold: u32,
    /// only one of these structures can be built
    pub is_exclusive: bool,
}

#[derive(Resource, Deref)]
pub struct StructureCosts(pub HashMap<StructureType, StructureCost>);

impl Default for StructureCosts {
    fn default() -> Self {
        let costs: HashMap<_, _> = [
            // (
            //     StructureType::SmallHouse,
            //     StructureCost {
            //         days: 1,
            //         workers: 5,
            //         gold: 25,
            //         is_exclusive: false,
            //     },
            // ),
            (
                StructureType::House,
                StructureCost {
                    turns: 1,
                    workers: 10,
                    gold: 50,
                    is_exclusive: false,
                },
            ),
            // (
            //     StructureType::StrongHouse,
            //     StructureCost {
            //         days: 1,
            //         workers: 15,
            //         gold: 100,
            //         is_exclusive: false,
            //     },
            // ),
            (
                StructureType::Tavern,
                StructureCost {
                    turns: 1,
                    workers: 10,
                    gold: 75,
                    is_exclusive: true,
                },
            ),
            (
                StructureType::ArcherTower,
                StructureCost {
                    turns: 1,
                    workers: 5,
                    gold: 25,
                    is_exclusive: false,
                },
            ),
            (
                StructureType::Blacksmith,
                StructureCost {
                    turns: 1,
                    workers: 10,
                    gold: 125,
                    is_exclusive: true,
                },
            ),
        ]
        .into_iter()
        .collect();

        Self(costs.into_iter().collect())
    }
}

#[derive(Component)]
pub struct BuildingPanel;

#[derive(Component)]
pub struct StructureDetail;

pub fn building_panel_layout(mut commands: Commands) {
    commands.ui_builder(UiRoot).row(|ui| {
        ui.insert((BuildingPanel, StateScoped(Screen::Playing)));
        ui.style()
            .width(Val::Percent(100.))
            .height(Val::Percent(100.))
            .justify_content(JustifyContent::End);

        ui.column(|ui| {
            ui.style()
                .padding(UiRect::all(Val::Px(10.)))
                .justify_content(JustifyContent::Start);

            ui.row(|_| {}).style().height(Val::Px(60.));

            ui.row(|ui| {
                ui.style().justify_content(JustifyContent::End);
                ui.label(LabelConfig::from("Build"))
                    .style()
                    .font_size(BIG_TEXT_SIZE);
            });

            ui.row(|ui| {
                for building_type in StructureType::iter() {
                    ui.container(ButtonBundle::default(), |ui| {
                        ui.style()
                            .flex_shrink(1.)
                            .flex_grow(0.)
                            .align_self(AlignSelf::End);

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
                        .padding(UiRect::all(Val::Px(4.)));

                        ui.column(|ui| {
                            ui.style().justify_content(JustifyContent::SpaceBetween);
                            ui.icon("icons/".to_owned() + building_type.tile_texture() + ".png")
                                .style()
                                .height(Val::Px(30.))
                                .width(Val::Px(30.));
                        });
                    });
                }
            });

            ui.column(|ui| {
                ui.style().padding(UiRect {
                    left: Val::Px(20.),
                    right: Val::Px(5.),
                    top: Val::Px(10.),
                    ..default()
                });

                ui.row(|ui| {
                    ui.style().justify_content(JustifyContent::End);
                    ui.insert(StructureDetail);
                });
            });
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

pub fn update_structure_detail(
    mut commands: Commands,
    q_structure_detail: Query<Entity, With<StructureDetail>>,
    selected_structure: Res<SelectedStructueType>,
    costs: Res<StructureCosts>,
) {
    let Ok(entity) = q_structure_detail.get_single() else {
        return;
    };
    commands.entity(entity).despawn_descendants();

    let Some(structure_type) = selected_structure.0 else {
        return;
    };

    let Some(cost) = costs.get(&structure_type) else {
        return;
    };

    commands.ui_builder(entity).column(|ui| {
        ui.style()
            .padding(UiRect::all(Val::Px(10.)))
            .border(UiRect::all(Val::Px(2.)))
            .border_color(Color::WHITE.with_alpha(0.3))
            .background_color(Color::BLACK.with_alpha(0.3))
            .row_gap(Val::Px(10.));

        ui.row(|ui| {
            ui.label(LabelConfig::from(structure_type.name()))
                .style()
                .font_size(TEXT_SIZE);
        });

        ui.style().row_gap(Val::Px(10.));
        for (icon, value) in [
            ("icons/gold_coins.png", cost.gold.to_string()),
            ("icons/population.png", cost.workers.to_string()),
            ("icons/hourglass.png", cost.turns.to_string()),
        ] {
            ui.row(|ui| {
                ui.style().justify_content(JustifyContent::End);
                ui.icon(icon).style().width(ICON_SIZE).height(ICON_SIZE);

                ui.label(LabelConfig::from(value))
                    .style()
                    .font_size(TEXT_SIZE);
            });
        }

        ui.row(|ui| {
            ui.label(LabelConfig::from("Select tile to build."))
                .style()
                .font_size(TEXT_SIZE * 0.5);
        });

        ui.row(|ui| {
            ui.container(ButtonBundle::default(), |ui| {
                ui.label(LabelConfig::from("Cancel"))
                    .style()
                    .font_size(TEXT_SIZE);
            })
            .insert((
                CancelBuildButton,
                InteractionPalette {
                    none: css::RED.into(),
                    hovered: css::INDIAN_RED.into(),
                    pressed: css::DARK_RED.into(),
                },
            ))
            .style()
            .padding(UiRect::all(Val::Px(4.)))
            .border_radius(BorderRadius::all(Val::Px(4.)));
        });
    });
}

pub fn cancel_build_btn_interaction(
    q_interactions: Query<&Interaction, (Changed<Interaction>, With<CancelBuildButton>)>,
    mut selected_structure: ResMut<SelectedStructueType>,
) {
    for i in q_interactions.iter() {
        if *i == Interaction::Pressed {
            selected_structure.0 = None;
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

#[derive(Component)]
pub struct BuildingSite;

pub fn spawn_in_progress_building(
    mut commands: Commands,
    mut events: EventReader<TilePressedEvent>,
    //mut village_map: ResMut<VillageMap>,
    tile_set: Res<TileSet>,
    selected_structure_type: Res<SelectedStructueType>,
    structure_cost: Res<StructureCosts>,
    population: Res<VillagePopulation>,
    mut working_population: ResMut<VillageEmployment>,
    mut gold: ResMut<VillageGold>,
    structure_query: Query<&StructureType>,
    mut game: Game,
) {
    let Some(TilePressedEvent(tile)) = events.read().last() else {
        return;
    };

    let Some(structure_type) = selected_structure_type.0 else {
        return;
    };

    let Some(cost) = structure_cost.get(&structure_type) else {
        return;
    };

    if game.is_occupied(*tile) {
        return;
    }

    let sites = game.find_tiles_that_can_be_built_on();

    if !sites.contains(tile) {
        return;
    }

    if cost.is_exclusive {
        for s in structure_query.iter() {
            if *s == structure_type {
                return;
            }
        }
    }

    if gold.0 < cost.gold {
        return;
    }

    if population.0 < working_population.0 + cost.workers {
        return;
    }

    gold.0 -= cost.gold;
    working_population.0 += cost.workers;

    let object_translation = tile_coord_translation(tile.x() as f32, tile.y() as f32, 2.0);
    let id = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    anchor: TILE_ANCHOR,
                    color: Color::WHITE.with_alpha(0.85),
                    ..Default::default()
                },
                transform: Transform::from_translation(object_translation),
                texture: tile_set.get(structure_type.tile_texture()),
                ..default()
            },
            StateScoped(Screen::Playing),
            structure_type,
            RemainingConstructionTurns(cost.turns),
            ConstructionWorkers(cost.workers),
            BuildingSite,
            Structure,
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        cost.turns.to_string(),
                        TextStyle {
                            font_size: 100.,
                            ..Default::default()
                        },
                    ),
                    transform: Transform::from_translation(0.01 * Vec3::Z),
                    ..Default::default()
                },
                BuildingProgressLabel,
            ));
        })
        .with_children(|builder| {
            builder.spawn((SpriteBundle {
                sprite: Sprite {
                    anchor: super::tile_set::TILE_ANCHOR,
                    color: Color::WHITE.with_alpha(0.75),
                    ..Default::default()
                },
                transform: Transform::from_translation(-0.01 * Vec3::Z),
                texture: tile_set.get("border_thick"),
                ..Default::default()
            },));
        })
        .id();

    game.insert(*tile, id);
}

pub fn update_building_progress(
    mut commands: Commands,
    mut building_query: Query<(
        Entity,
        &mut RemainingConstructionTurns,
        &StructureType,
        &ConstructionWorkers,
    )>,
    mut village_map: ResMut<VillageMap>,
    tile_set: Res<TileSet>,
    mut working_population: ResMut<VillageEmployment>,
) {
    for (e, mut b, s, w) in building_query.iter_mut() {
        b.0 = b.0.saturating_sub(1);
        if b.0 == 0 {
            working_population.0 = working_population.0.saturating_sub(w.0);
            commands.entity(e).despawn_recursive();
            let Some(tile) = village_map.actors.locate(e) else {
                continue;
            };
            let object_translation = tile_coord_translation(tile.x() as f32, tile.y() as f32, 2.);
            let mut object_entity = commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        anchor: TILE_ANCHOR,
                        ..Default::default()
                    },
                    texture: tile_set.get(s.tile_texture()),
                    transform: Transform::from_translation(object_translation),
                    ..default()
                },
                tile,
                PickableTile,
                StateScoped(Screen::Playing),
                StructureBundle::default(),
                SpawnAnimation::new(object_translation),
                *s,
            ));
            println!("building structure type = {s:?}");
            println!("entity = {:?}", object_entity.id());
            match s {
                StructureType::Tavern => {
                    println!("Insert Tavern");
                    object_entity.insert(Tavern);
                }
                // StructureType::SmallHouse => {
                //     object_entity.insert(House);
                // }
                StructureType::House => {
                    object_entity.insert(House);
                }
                // StructureType::StrongHouse => {
                //     object_entity.insert(House);
                // }
                StructureType::ArcherTower => {
                    object_entity.insert(ArcherTower);
                }
                StructureType::Blacksmith => {
                    object_entity.insert(Blacksmith);
                }
            };

            village_map.actors.set(tile, object_entity.id());
        }
    }
}

pub fn update_building_progress_labels(
    mut building_query: Query<(&mut Text, &Parent), With<BuildingProgressLabel>>,
    remaining: Query<&RemainingConstructionTurns>,
    mut events: EventReader<EndDayTurn>,
) {
    if events.read().last().is_some() {
        for (mut t, p) in building_query.iter_mut() {
            let Ok(r) = remaining.get(p.get()) else {
                continue;
            };
            t.sections[0].value = r.0.to_string();
        }
    }
}
