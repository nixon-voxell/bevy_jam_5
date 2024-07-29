use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use sickle_ui::prelude::*;

use crate::game::components::Tavern;
use crate::game::constants::UPGRADE_COST;
use crate::game::construction::BuildingSite;
use crate::game::construction::StructureType;
use crate::game::inventory::MaxInventorySize;
use crate::game::map::VillageMap;
use crate::game::resources::VillageGold;
use crate::game::selection::ObjectPressedEvent;
use crate::game::selection::SelectedUnit;
use crate::game::unit::player::TilePressedEvent;
use crate::game::unit::Health;
use crate::game::unit::MaxHealth;
use crate::game::unit::Movement;
use crate::game::unit::PlayerUnit;
use crate::game::unit::Structure;
use crate::game::unit::UnitName;
use crate::game::unit_list::PlayerUnitList;
use crate::game::MERCHANT_ITEMS;
use crate::game::MODAL_Z_LAYER;
use crate::screen::playing::GameState;
use crate::ui::palette::LABEL_SIZE;
use crate::ui::prelude::InteractionPalette;

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

pub fn tavern_modal_layout(
    mut commands: Commands,
    player_unit_list: Res<PlayerUnitList>,
    unit_query: Query<(Entity, &UnitName)>,
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
                        .margin(UiRect::all(Val::Px(16.)));
                    ui.icon("icons/mug_of_beer.png")
                        .style()
                        .width(Val::Px(64.))
                        .height(Val::Px(64.));

                    ui.column(|ui| {
                        for entity in player_unit_list.0.iter() {
                            if let Ok((entity, name, ..)) = unit_query.get(*entity) {
                                ui.container(ButtonBundle::default(), |ui| {
                                    ui.insert(TavernButton(entity));

                                    ui.style()
                                        .margin(UiRect::all(Val::Px(2.)))
                                        .border(UiRect::all(Val::Px(2.)))
                                        .border_color(Color::WHITE)
                                        .justify_content(JustifyContent::Start);
                                    ui.label(LabelConfig::from(name.0.clone()));
                                })
                                .insert(InteractionPalette {
                                    none: css::BLACK.into(),
                                    hovered: css::DARK_RED.into(),
                                    pressed: css::INDIAN_RED.into(),
                                });
                            }
                        }
                    });

                    ui.row(|ui| {
                        ui.column(|ui| {
                            ui.style().row_gap(Val::Px(10.));
                            ui.label(LabelConfig::from("")).insert(NameLabel);
                            ui.row(|ui| {
                                ui.label(LabelConfig::from("Movement:"));
                                ui.label(LabelConfig::from("")).insert(MovementLabel);
                                ui.row(|ui| {}).style().width(Val::Px(20.));
                                ui.container(ButtonBundle::default(), |ui| {
                                    ui.insert(TavernUpgrade::AddMovement)
                                        .style()
                                        .margin(UiRect::all(Val::Px(2.)))
                                        .border(UiRect::all(Val::Px(2.)))
                                        .border_color(Color::WHITE)
                                        .justify_content(JustifyContent::Start);
                                    ui.label(LabelConfig::from(format!(
                                        "+movement {UPGRADE_COST} gold"
                                    )));
                                })
                                .insert(InteractionPalette {
                                    none: css::BLACK.into(),
                                    hovered: css::DARK_RED.into(),
                                    pressed: css::INDIAN_RED.into(),
                                });
                            });

                            ui.row(|ui| {
                                ui.label(LabelConfig::from("Health:"));
                                ui.label(LabelConfig::from("")).insert(HealthLabel);
                                ui.row(|ui| {}).style().width(Val::Px(20.));
                                ui.container(ButtonBundle::default(), |ui| {
                                    ui.insert(TavernUpgrade::AddHealth)
                                        .style()
                                        .margin(UiRect::all(Val::Px(2.)))
                                        .border(UiRect::all(Val::Px(2.)))
                                        .border_color(Color::WHITE)
                                        .justify_content(JustifyContent::Start);
                                    ui.label(LabelConfig::from(format!(
                                        "+health {UPGRADE_COST} gold"
                                    )));
                                })
                                .insert(InteractionPalette {
                                    none: css::BLACK.into(),
                                    hovered: css::DARK_RED.into(),
                                    pressed: css::INDIAN_RED.into(),
                                });
                            });

                            ui.row(|ui| {
                                ui.label(LabelConfig::from("Item Slots:"));
                                ui.label(LabelConfig::from("")).insert(ItemSlotLabel);
                                ui.row(|_| {}).style().width(Val::Px(20.));
                                ui.container(ButtonBundle::default(), |ui| {
                                    ui.insert(TavernUpgrade::AddItemSlot)
                                        .style()
                                        .margin(UiRect::all(Val::Px(2.)))
                                        .border(UiRect::all(Val::Px(2.)))
                                        .border_color(Color::WHITE)
                                        .justify_content(JustifyContent::Start);
                                    ui.label(LabelConfig::from(format!(
                                        "+item slot {UPGRADE_COST} gold"
                                    )));
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
    query: Query<(&UnitName, &Movement, &MaxHealth, &MaxInventorySize)>,
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
            t.sections[0].value = format!("{}", n.0);
        }
        for mut t in m_query.iter_mut() {
            t.sections[0].value = format!("{}", m.0);
        }
        for mut t in h_query.iter_mut() {
            t.sections[0].value = format!("{}", h.0);
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
    mut stats_query: Query<(
        &mut Movement,
        &mut Health,
        &mut MaxHealth,
        &mut MaxInventorySize,
    )>,
) {
    let Ok((mut m, mut h, mut mh, mut s)) = stats_query.get_mut(subject.0) else {
        return;
    };

    fn upgrade(value: &mut u32) -> bool {
        let n = *value;
        *value = (*value + 1).min(5);
        n == *value
    }

    for (i, u) in upgrade_query.iter() {
        if *i == Interaction::Pressed {
            if UPGRADE_COST <= gold.0 {
                if match u {
                    TavernUpgrade::AddMovement => upgrade(&mut m.0),
                    TavernUpgrade::AddHealth => {
                        if upgrade(&mut mh.0) {
                            h.0 = (h.0 + 1).min(mh.0);
                            true
                        } else {
                            false
                        }
                    }
                    TavernUpgrade::AddItemSlot => upgrade(&mut s.0),
                } {
                    gold.0 -= UPGRADE_COST;
                }
            }
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
