use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::utils::HashMap;
use sickle_ui::prelude::*;

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

#[derive(Component, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum StructureType {
    SmallHouse,
    House,
    StrongHouse,
    Tavern,
    ArcherTower,
    Blacksmith,
}

impl StructureType {
    pub const ALL: [Self; 6] = [
        Self::SmallHouse,
        Self::House,
        Self::StrongHouse,
        Self::Tavern,
        Self::ArcherTower,
        Self::Blacksmith,
    ];

    pub fn name(&self) -> &str {
        match self {
            StructureType::SmallHouse => "small house",
            StructureType::House => "house",
            StructureType::StrongHouse => "strong house",
            StructureType::Tavern => "tavern",
            StructureType::ArcherTower => "tower",
            StructureType::Blacksmith => "blacksmith",
        }
    }

    pub fn tile_texture(&self) -> &str {
        match self {
            StructureType::SmallHouse => "house1",
            StructureType::House => "house1",
            StructureType::StrongHouse => "house1",
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
    /// only one of these structures can be built
    pub is_exclusive: bool,
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
                    is_exclusive: false,
                },
            ),
            (
                StructureType::House,
                StructureCost {
                    turns: 4,
                    workers: 10,
                    gold: 50,
                    is_exclusive: false,
                },
            ),
            (
                StructureType::StrongHouse,
                StructureCost {
                    turns: 6,
                    workers: 15,
                    gold: 100,
                    is_exclusive: false,
                },
            ),
            (
                StructureType::Tavern,
                StructureCost {
                    turns: 5,
                    workers: 10,
                    gold: 75,
                    is_exclusive: true,
                },
            ),
            (
                StructureType::ArcherTower,
                StructureCost {
                    turns: 3,
                    workers: 5,
                    gold: 25,
                    is_exclusive: false,
                },
            ),
            (
                StructureType::Blacksmith,
                StructureCost {
                    turns: 7,
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

pub fn building_panel_layout(mut commands: Commands, costs: Res<StructureCosts>) {
    commands.ui_builder(UiRoot).row(|ui| {
        ui.insert(StateScoped(Screen::Playing))
            .insert(BuildingPanel)
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
    mut events: EventReader<EndDayTurn>,
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
    if events.read().last().is_some() {
        for (e, mut b, s, w) in building_query.iter_mut() {
            b.0 = b.0.saturating_sub(1);
            if b.0 == 0 {
                working_population.0 = working_population.0.saturating_sub(w.0);
                commands.entity(e).despawn_recursive();
                let Some(tile) = village_map.actors.locate(e) else {
                    continue;
                };
                let object_translation =
                    tile_coord_translation(tile.x() as f32, tile.y() as f32, 2.);
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
                    s.clone(),
                ));
                println!("building structure type = {s:?}");
                println!("entity = {:?}", object_entity.id());
                match s {
                    StructureType::Tavern => {
                        println!("Insert Tavern");
                        object_entity.insert(Tavern);
                    }
                    StructureType::SmallHouse => {
                        object_entity.insert(House);
                    }
                    StructureType::House => {
                        object_entity.insert(House);
                    }
                    StructureType::StrongHouse => {
                        object_entity.insert(House);
                    }
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
