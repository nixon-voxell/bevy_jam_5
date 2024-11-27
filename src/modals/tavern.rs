use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use sickle_ui::prelude::*;

use crate::game::actors::player::spawn_player_unit;
use crate::game::actors::stats::{ActorName, Health, Movement};
use crate::game::actors::AvailableActorNames;
use crate::game::actors_list::PlayerActorList;
use crate::game::components::Tavern;
use crate::game::constants::{BIG_TEXT_SIZE, RECRUIT_COST, TAVERN_FONT_SIZE, UPGRADE_COST};
use crate::game::inventory::{Inventory, MaxInventorySize};
use crate::game::resources::VillageGold;
use crate::game::selection::ObjectPressedEvent;
use crate::game::MODAL_Z_LAYER;
use crate::screen::playing::GameState;
use crate::ui::palette::LABEL_SIZE;
use crate::ui::prelude::*;

#[derive(Component, Default)]
pub struct BuyButton(pub Option<Entity>);

#[derive(Component)]
pub struct ItemButton(pub Entity);

#[derive(Component)]
pub struct ExitTavernButton;

#[derive(Component)]
pub struct TavernButton(pub Entity);

/// Each can be bought twice
#[derive(Component)]
pub enum TavernUpgrade {
    AddMovement,
    AddHealth,
    AddItemSlot,
}

#[derive(Component)]
pub struct MovementLabel;

#[derive(Component)]
pub struct HealthLabel;

#[derive(Component)]
pub struct ItemSlotLabel;

#[derive(Component)]
pub struct NameLabel;

#[derive(Component)]
pub struct RecruitButton;

#[derive(Component)]
pub struct TavernActorList;

pub fn spawn_hero_button(ui: &mut UiBuilder<Entity>, entity: Entity, name: String) {
    ui.container(ButtonBundle::default(), |ui| {
        ui.insert(TavernButton(entity));

        ui.style()
            .margin(UiRect::all(Val::Px(2.)))
            .padding(UiRect::all(Val::Px(2.)))
            .border(UiRect::all(Val::Px(2.)))
            .border_color(Color::WHITE)
            .justify_content(JustifyContent::Start);
        ui.label(LabelConfig::from(name))
            .style()
            .font_size(TAVERN_FONT_SIZE);
    })
    .insert(InteractionPalette {
        none: css::BLACK.into(),
        hovered: css::DARK_RED.into(),
        pressed: css::INDIAN_RED.into(),
    });
}

pub fn tavern_modal_layout(
    mut commands: Commands,
    player_unit_list: Res<PlayerActorList>,
    unit_query: Query<(Entity, &ActorName)>,
    mut subject: ResMut<TavernSubject>,
) {
    subject.0 = player_unit_list
        .0
        .first()
        .copied()
        .unwrap_or(Entity::PLACEHOLDER);

    commands
        .ui_builder(UiRoot)
        .column(|ui| {
            ui.row(|ui| {
                ui.style()
                    .padding(UiRect::all(Val::Px(18.)))
                    .border(UiRect::all(Val::Px(2.)))
                    .border_color(Color::WHITE)
                    .border_radius(BorderRadius::all(Val::Px(16.)))
                    .background_color(Color::BLACK)
                    .width(Val::Px(480.))
                    .height(Val::Px(600.))
                    .justify_content(JustifyContent::Center);
                ui.column(|ui| {
                    ui.style()
                        .align_items(AlignItems::Center)
                        .row_gap(Val::Px(20.));
                    ui.label(LabelConfig::from("Tavern"))
                        .style()
                        .margin(UiRect::all(Val::Px(16.)))
                        .font_size(BIG_TEXT_SIZE);
                    ui.icon("icons/mug_of_beer.png")
                        .style()
                        .width(Val::Px(64.))
                        .height(Val::Px(64.));

                    ui.column(|ui| {
                        ui.insert(TavernActorList);
                        for entity in player_unit_list.0.iter() {
                            if let Ok((entity, name, ..)) = unit_query.get(*entity) {
                                spawn_hero_button(ui, entity, name.0.clone());
                            }
                        }
                    });

                    if player_unit_list.0.len() <= 5 {
                        ui.row(|ui| {
                            ui.container(ButtonBundle::default(), |ui| {
                                ui.insert(RecruitButton);

                                ui.style()
                                    .margin(UiRect::all(Val::Px(2.)))
                                    .padding(UiRect::all(Val::Px(2.)))
                                    .border(UiRect::all(Val::Px(2.)))
                                    .border_color(Color::WHITE)
                                    .justify_content(JustifyContent::Start);
                                ui.label(LabelConfig::from(format!("Recruit {RECRUIT_COST} gold")))
                                    .style()
                                    .font_size(TAVERN_FONT_SIZE);
                            })
                            .insert(InteractionPalette {
                                none: css::BLACK.into(),
                                hovered: css::DARK_RED.into(),
                                pressed: css::INDIAN_RED.into(),
                            });
                        });
                    }

                    ui.row(|ui| {
                        ui.column(|ui| {
                            ui.style().row_gap(Val::Px(10.));
                            ui.label(LabelConfig::from(""))
                                .insert(NameLabel)
                                .style()
                                .font_size(TAVERN_FONT_SIZE);
                            ui.row(|ui| {
                                ui.label(LabelConfig::from("Movement:"))
                                    .style()
                                    .font_size(TAVERN_FONT_SIZE);
                                ui.label(LabelConfig::from(""))
                                    .insert(MovementLabel)
                                    .style()
                                    .font_size(TAVERN_FONT_SIZE);
                                ui.row(|_| {}).style().width(Val::Px(20.));
                                ui.container(ButtonBundle::default(), |ui| {
                                    ui.insert(TavernUpgrade::AddMovement)
                                        .style()
                                        .margin(UiRect::all(Val::Px(2.)))
                                        .padding(UiRect::all(Val::Px(2.)))
                                        .border(UiRect::all(Val::Px(2.)))
                                        .border_color(Color::WHITE)
                                        .justify_content(JustifyContent::Start);
                                    ui.label(LabelConfig::from(format!(
                                        "+movement {UPGRADE_COST} gold"
                                    )))
                                    .style()
                                    .font_size(TAVERN_FONT_SIZE);
                                })
                                .insert(InteractionPalette {
                                    none: css::BLACK.into(),
                                    hovered: css::DARK_RED.into(),
                                    pressed: css::INDIAN_RED.into(),
                                });
                            });

                            ui.row(|ui| {
                                ui.label(LabelConfig::from("Health:"))
                                    .style()
                                    .font_size(TAVERN_FONT_SIZE);
                                ui.label(LabelConfig::from(""))
                                    .insert(HealthLabel)
                                    .style()
                                    .font_size(TAVERN_FONT_SIZE);
                                ui.row(|_| {}).style().width(Val::Px(20.));
                                ui.container(ButtonBundle::default(), |ui| {
                                    ui.insert(TavernUpgrade::AddHealth)
                                        .style()
                                        .margin(UiRect::all(Val::Px(2.)))
                                        .padding(UiRect::all(Val::Px(2.)))
                                        .border(UiRect::all(Val::Px(2.)))
                                        .border_color(Color::WHITE)
                                        .justify_content(JustifyContent::Start);
                                    ui.label(LabelConfig::from(format!(
                                        "+health {UPGRADE_COST} gold"
                                    )))
                                    .style()
                                    .font_size(TAVERN_FONT_SIZE);
                                })
                                .insert(InteractionPalette {
                                    none: css::BLACK.into(),
                                    hovered: css::DARK_RED.into(),
                                    pressed: css::INDIAN_RED.into(),
                                });
                            });

                            ui.row(|ui| {
                                ui.label(LabelConfig::from("Item Slots:"))
                                    .style()
                                    .font_size(TAVERN_FONT_SIZE);
                                ui.label(LabelConfig::from(""))
                                    .insert(ItemSlotLabel)
                                    .style()
                                    .font_size(TAVERN_FONT_SIZE);
                                ui.row(|_| {}).style().width(Val::Px(20.));
                                ui.container(ButtonBundle::default(), |ui| {
                                    ui.insert(TavernUpgrade::AddItemSlot)
                                        .style()
                                        .margin(UiRect::all(Val::Px(2.)))
                                        .padding(UiRect::all(Val::Px(2.)))
                                        .border(UiRect::all(Val::Px(2.)))
                                        .border_color(Color::WHITE)
                                        .justify_content(JustifyContent::Start);
                                    ui.label(LabelConfig::from(format!(
                                        "+item slot {UPGRADE_COST} gold"
                                    )))
                                    .style()
                                    .font_size(TAVERN_FONT_SIZE);
                                })
                                .insert(InteractionPalette {
                                    none: css::BLACK.into(),
                                    hovered: css::DARK_RED.into(),
                                    pressed: css::INDIAN_RED.into(),
                                });
                            });
                        });
                    });
                });
                // Close button
                ui.container(ButtonBundle::default(), |ui| {
                    ui.label(LabelConfig::from("x"))
                        .style()
                        .font_size(LABEL_SIZE);
                })
                .insert((
                    InteractionPalette {
                        none: Color::BLACK.with_alpha(0.0),
                        hovered: Color::BLACK.with_alpha(0.0),
                        pressed: Color::BLACK.with_alpha(0.0),
                    },
                    ExitTavernButton,
                ))
                .style()
                .position_type(PositionType::Absolute)
                .top(Val::Px(16.))
                .right(Val::Px(16.));
            });
        })
        .insert(StateScoped(GameState::Tavern))
        .style()
        .focus_policy(FocusPolicy::Block)
        .z_index(ZIndex::Global(MODAL_Z_LAYER))
        .width(Val::Percent(100.))
        .height(Val::Percent(100.))
        .background_color(Color::srgba(0.25, 0.25, 0.25, 0.75))
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center);
}

pub fn exit_tavern_btn_interaction(
    q_interactions: Query<&Interaction, (Changed<Interaction>, With<ExitTavernButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for interaction in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            next_game_state.set(GameState::BuildingTurn);
        }
    }
}

pub fn enter_tavern_modal(
    mut events: EventReader<ObjectPressedEvent>,
    query: Query<&Tavern>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(ObjectPressedEvent(entity)) = events.read().last().copied() else {
        return;
    };

    if query.contains(entity) && *state.get() == GameState::BuildingTurn {
        next_state.set(GameState::Tavern);
    }
}

#[derive(Resource)]
pub struct TavernSubject(pub Entity);

impl Default for TavernSubject {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}

pub fn update_slot_labels(
    subject: Res<TavernSubject>,
    query: Query<(&ActorName, &Movement, &Health, &MaxInventorySize)>,
    mut n_query: Query<
        &mut Text,
        (
            With<NameLabel>,
            Without<MovementLabel>,
            Without<HealthLabel>,
            Without<ItemSlotLabel>,
        ),
    >,

    mut m_query: Query<
        &mut Text,
        (
            Without<NameLabel>,
            With<MovementLabel>,
            Without<HealthLabel>,
            Without<ItemSlotLabel>,
        ),
    >,
    mut h_query: Query<
        &mut Text,
        (
            Without<NameLabel>,
            Without<MovementLabel>,
            With<HealthLabel>,
            Without<ItemSlotLabel>,
        ),
    >,
    mut s_query: Query<
        &mut Text,
        (
            Without<NameLabel>,
            Without<MovementLabel>,
            Without<HealthLabel>,
            With<ItemSlotLabel>,
        ),
    >,
) {
    if let Ok((n, m, h, s)) = query.get(subject.0) {
        for mut t in n_query.iter_mut() {
            t.sections[0].value = n.0.clone();
        }
        for mut t in m_query.iter_mut() {
            t.sections[0].value = format!("{}", m.0);
        }
        for mut t in h_query.iter_mut() {
            t.sections[0].value = format!("{}", h.value);
        }
        for mut t in s_query.iter_mut() {
            t.sections[0].value = format!("{}", s.0);
        }
    }
}

pub fn upgrade_buttons(
    mut gold: ResMut<VillageGold>,
    subject: Res<TavernSubject>,
    upgrade_query: Query<(&Interaction, &TavernUpgrade), Changed<Interaction>>,
    mut stats_query: Query<(&mut Movement, &mut Health, &mut Inventory)>,
) {
    let Ok((mut m, mut h, mut s)) = stats_query.get_mut(subject.0) else {
        return;
    };

    fn upgrade(value: &mut u32) -> bool {
        let n = *value;
        *value = (*value + 1).min(5);
        n == *value
    }

    for (_, u) in upgrade_query
        .iter()
        .filter(|(&i, _)| i == Interaction::Pressed)
    {
        if UPGRADE_COST > gold.0 {
            continue;
        }

        if match u {
            TavernUpgrade::AddMovement => upgrade(&mut m.0),
            TavernUpgrade::AddHealth => {
                if upgrade(&mut h.max) {
                    h.value = (h.value + 1).min(h.max);
                    true
                } else {
                    false
                }
            }
            TavernUpgrade::AddItemSlot => {
                if s.slot_count() < 5 {
                    s.add_slot();
                    true
                } else {
                    false
                }
            }
        } {
            gold.0 -= UPGRADE_COST;
        }
    }
}

pub fn tavern_button(
    mut subject: ResMut<TavernSubject>,
    i_q: Query<(&Interaction, &TavernButton)>,
) {
    for (i, b) in i_q.iter() {
        if *i == Interaction::Pressed {
            subject.0 = b.0;
        }
    }
}

pub fn recruit_button(
    r_q: Query<&Interaction, (With<RecruitButton>, Changed<Interaction>)>,
    mut commands: Commands,
    mut gold: ResMut<VillageGold>,
    mut player_unit_list: ResMut<PlayerActorList>,
    mut names: ResMut<AvailableActorNames>,
    t_q: Query<Entity, With<TavernActorList>>,
) {
    if player_unit_list.0.len() < 5 {
        for _ in r_q.iter().filter(|&&i| i == Interaction::Pressed) {
            if RECRUIT_COST > gold.0 {
                continue;
            }

            gold.0 -= RECRUIT_COST;
            let name = names.next_name();
            let id = spawn_player_unit(&mut commands, name.clone());
            player_unit_list.0.push(id);
            for entity in t_q.iter() {
                spawn_hero_button(&mut commands.ui_builder(entity), id, name.clone())
            }
        }
    }
}
