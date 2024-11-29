//! The screen state for the main game loop.

use bevy::color::palettes::css;
use bevy::ecs::entity::EntityHashMap;
use bevy::prelude::*;
use sickle_ui::prelude::*;

use super::Screen;
use crate::game::actors::AvailableActorNames;
use crate::game::constants::{INITIAL_GOLD, INITIAL_POPULATION, UNIT_LIST_ZINDEX};
use crate::game::construction::{
    build_btn_interaction, building_panel_layout, spawn_in_progress_building, update_build_panel,
    update_building_progress, update_building_progress_labels, BuildingPanel, StructureCosts,
};
use crate::game::cycle::{DayCycle, EndDeployment, EndTurn, Season, TimeOfDay, Turn};
use crate::game::deployment::{
    deployment_setup, deployment_zone_visualization, is_deployment_ready,
};
use crate::game::events::{EndDayTurn, SelectStructureTypeEvent};
use crate::game::level::load_level;
use crate::game::resources::{
    SelectedStructueType, VillageEmployment, VillageGold, VillagePopulation,
};

use crate::game::actors::player::{add_starting_player_units, move_unit, reset_unit_turn_states};
use crate::game::selection::{dispatch_object_pressed, SelectedTiles};

use crate::game::actors_list::{
    actor_list_layout, inventory_list_layout, inventory_list_layout_vis,
    select_item_btn_interaction, select_player_actor_btn_interaction, update_actor_list_container,
    update_inventory_icons, update_selected_actor_name_label, ItemSlotIcons, PlayerActorList,
};
use crate::game::WatchRes;
use crate::game::{assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack};

use crate::modals::tavern::{
    enter_tavern_modal, exit_tavern_btn_interaction, recruit_button, tavern_button,
    tavern_modal_layout, update_slot_labels, upgrade_buttons, TavernSubject,
};

use crate::modals::merchant::MerchantModalPlugin;
use crate::ui::icon_set::IconSet;

use crate::ui::interaction::apply_interaction_palette;
use crate::ui::{palette::*, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(MerchantModalPlugin)
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .init_resource::<DisplayCache>()
        .init_resource::<AvailableActorNames>()
        .init_resource::<PlayerActorList>()
        .init_resource::<StructureCosts>()
        .init_resource::<SelectedStructueType>()
        .init_resource::<TavernSubject>()
        .init_resource::<ItemSlotIcons>()
        .add_event::<SelectStructureTypeEvent>()
        .add_systems(OnEnter(Screen::Playing), enter_playing)
        .add_systems(OnEnter(Screen::Playing), add_starting_player_units)
        .add_systems(OnEnter(GameState::Tavern), tavern_modal_layout)
        .add_systems(
            Update,
            exit_tavern_btn_interaction.run_if(in_state(GameState::Tavern)),
        )
        .add_systems(OnExit(Screen::Playing), exit_playing)
        .add_systems(OnEnter(Screen::Playing), building_panel_layout)
        .add_systems(
            Update,
            reset_unit_turn_states.run_if(in_state(Screen::Playing)),
        )
        .add_systems(
            OnEnter(GameState::Deployment),
            (deployment_setup, deployment_zone_visualization).chain(),
        )
        .add_systems(
            OnExit(GameState::Deployment),
            |mut selected_tiles: ResMut<SelectedTiles>| {
                selected_tiles.tiles.clear();
            },
        )
        .add_systems(
            OnEnter(TimeOfDay::Day),
            |mut selected_tiles: ResMut<SelectedTiles>| {
                selected_tiles.tiles.clear();
            },
        )
        .add_systems(
            OnEnter(GameState::Deployment),
            (
                hide_all_with::<EndTurnButton>,
                hide_all_with::<OpenMerchantButton>,
            ),
        )
        .add_systems(OnEnter(Screen::Playing), |mut commands: Commands| {
            commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
        })
        .add_systems(
            OnEnter(GameState::BuildingTurn),
            (
                show_all_with::<OpenMerchantButton>,
                (|mut commands: Commands| {
                    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
                })
                .run_if(in_state(Screen::Playing)),
            ),
        )
        .add_systems(
            Update,
            show_all_with::<FightButton>
                .run_if(in_state(GameState::Deployment).and_then(is_deployment_ready)),
        )
        .add_systems(
            OnExit(GameState::Deployment),
            (show_all_with::<EndTurnButton>, hide_all_with::<FightButton>),
        )
        .add_systems(
            OnEnter(GameState::EnemyTurn),
            hide_all_with::<EndTurnButton>,
        )
        .add_systems(OnExit(GameState::EnemyTurn), show_all_with::<EndTurnButton>)
        .add_systems(Update, move_unit.run_if(in_state(GameState::BattleTurn)))
        .add_systems(
            OnExit(TimeOfDay::Day),
            |mut s: ResMut<SelectedStructueType>| {
                s.0 = None;
            },
        )
        .add_systems(
            Update,
            (
                build_btn_interaction.run_if(in_state(GameState::BuildingTurn)),
                update_build_panel.run_if(resource_changed::<SelectedStructueType>),
            )
                .chain(),
        )
        .add_systems(
            Update,
            spawn_in_progress_building
                .run_if(in_state(Screen::Playing).and_then(in_state(GameState::BuildingTurn))),
        )
        .add_systems(
            OnEnter(TimeOfDay::Day),
            (
                update_building_progress,
                update_building_progress_labels.after(update_building_progress),
            )
                .chain()
                .run_if(in_state(Screen::Playing)),
        )
        .add_systems(Update, enter_tavern_modal.after(dispatch_object_pressed))
        .add_systems(
            Update,
            (
                tavern_button,
                upgrade_buttons,
                update_slot_labels,
                recruit_button,
            )
                .chain()
                .run_if(in_state(GameState::Tavern)),
        )
        .add_systems(
            OnEnter(GameState::BuildingTurn),
            show_all_with::<BuildingPanel>,
        )
        .add_systems(
            OnExit(GameState::BuildingTurn),
            hide_all_with::<BuildingPanel>,
        )
        .add_systems(
            Update,
            (
                inventory_list_layout_vis.run_if(in_state(Screen::Playing)),
                update_inventory_icons.run_if(in_state(Screen::Playing)),
                select_item_btn_interaction.run_if(in_state(Screen::Playing)),
            ),
        );

    app.add_systems(
        Update,
        (
            // exit_btn_interaction,
            end_turn_btn_interaction,
            fight_btn_interaction,
            open_merchant_btn_interaction,
            update_actor_list_container
                .run_if(in_state(Screen::Playing))
                .before(apply_interaction_palette),
            select_player_actor_btn_interaction,
            update_selected_actor_name_label,
        ),
    );
}

fn economy_status_layout(ui: &mut UiBuilder<Entity>) {
    ui.column(|ui| {
        ui.style().justify_content(JustifyContent::Center);
        ui.row(|ui| {
            ui.style().column_gap(Val::Px(40.));
            ui.row(|ui| {
                ui.style().column_gap(Val::Px(4.));
                ui.icon("icons/gold_coins.png")
                    .style()
                    .width(Val::Px(32.))
                    .height(Val::Px(32.));

                ui.label(LabelConfig::from(INITIAL_GOLD.to_string()))
                    .insert(WatchRes::<VillageGold>::default())
                    .style()
                    .font_size(LABEL_SIZE);
            });

            ui.row(|ui| {
                ui.style().column_gap(Val::Px(4.));
                ui.icon("icons/population.png")
                    .style()
                    .width(Val::Px(32.))
                    .height(Val::Px(32.));

                ui.label(LabelConfig::from(INITIAL_POPULATION.to_string()))
                    .insert(WatchRes::<VillageEmployment>::default())
                    .style()
                    .font_size(LABEL_SIZE);
                ui.label(LabelConfig::from("/"))
                    .style()
                    .font_size(LABEL_SIZE);
                ui.label(LabelConfig::from(INITIAL_POPULATION.to_string()))
                    .insert(WatchRes::<VillagePopulation>::default())
                    .style()
                    .font_size(LABEL_SIZE);
            });
        });
    });
}

fn enter_playing(
    mut commands: Commands,
    icon_set: Res<IconSet>,
    mut item_slots: ResMut<ItemSlotIcons>,
) {
    commands
        .ui_builder(UiRoot)
        .column(|ui| {
            ui.style()
                .width(Val::Percent(100.))
                .height(Val::Percent(100.))
                .padding(UiRect::all(Val::Px(20.)))
                .justify_content(JustifyContent::SpaceBetween);

            ui.row(|ui| {
                ui.style()
                    .justify_content(JustifyContent::SpaceBetween)
                    .align_items(AlignItems::Center)
                    .column_gap(Val::Px(20.));

                ui.label(LabelConfig::from("Season"))
                    .insert(WatchRes::<Season>::default())
                    .style()
                    .font_size(HEADER_SIZE);

                ui.column(|_| {}).style().flex_grow(1.);

                economy_status_layout(ui);

                ui.column(|_| {}).style().width(Val::Px(20.));
            });

            ui.row(|ui| {
                ui.style().justify_content(JustifyContent::SpaceBetween);
                ui.label(LabelConfig::from("Turn Until"))
                    .insert(WatchRes::<Turn>::default())
                    .style()
                    .align_self(AlignSelf::End)
                    .font_size(LABEL_SIZE);

                ui.column(|ui| {
                    ui.style()
                        .justify_content(JustifyContent::Center)
                        .justify_items(JustifyItems::Center);

                    ui.container(
                        ButtonBundle {
                            image: UiImage::new(icon_set.get("shop")),
                            ..default()
                        },
                        |_| {},
                    )
                    .insert((
                        InteractionPalette {
                            none: Color::WHITE,
                            hovered: Color::WHITE.lighter(0.4),
                            pressed: Color::WHITE,
                        },
                        OpenMerchantButton,
                    ))
                    .style()
                    .margin(UiRect::px(10., 10., 10., 20.))
                    .border_radius(BorderRadius::all(Val::Px(50.)))
                    .width(Val::Px(100.0))
                    .height(Val::Px(100.0));

                    ui.container(ButtonBundle::default(), |ui| {
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
                    .padding(UiRect::all(Val::Px(10.)))
                    .border_radius(BorderRadius::all(Val::Px(5.)));

                    ui.container(ButtonBundle { ..default() }, |ui| {
                        ui.label(LabelConfig::from("Fight"))
                            .insert(FightButton)
                            .style()
                            .font_size(LABEL_SIZE);
                    })
                    .insert((
                        InteractionPalette {
                            none: css::PURPLE.into(),
                            hovered: css::DARK_RED.into(),
                            pressed: css::INDIAN_RED.into(),
                        },
                        FightButton,
                    ))
                    .style()
                    .display(Display::None)
                    .padding(UiRect::all(Val::Px(10.)))
                    .border_radius(BorderRadius::all(Val::Px(5.)));
                });
            });
        })
        .insert(StateScoped(Screen::Playing));

    commands
        .ui_builder(UiRoot)
        .column(|ui| {
            ui.insert(StateScoped(Screen::Playing));
            ui.style()
                .z_index(UNIT_LIST_ZINDEX)
                .width(Val::Percent(100.))
                .height(Val::Percent(100.))
                .justify_content(JustifyContent::Center);
            ui.row(|ui| {
                ui.style()
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Start)
                    .margin(UiRect::left(Val::Px(10.)));
                ui.column(|ui| {
                    ui.style().row_gap(Val::Px(20.));
                    actor_list_layout(ui);

                    ui.row(|ui| {
                        item_slots.0 = inventory_list_layout(ui);
                    });
                });
            });
        })
        .style();
}

fn end_turn_btn_interaction(
    q_interactions: Query<&Interaction, (Changed<Interaction>, With<EndTurnButton>)>,
    mut next_turn_evt: EventWriter<EndTurn>,
    mut day_turn_evt: EventWriter<EndDayTurn>,
    state: Res<State<TimeOfDay>>,
) {
    for interaction in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            next_turn_evt.send(EndTurn);
            if *state.get() == TimeOfDay::Day {
                day_turn_evt.send(EndDayTurn);
            }
        }
    }
}

fn fight_btn_interaction(
    q_interactions: Query<&Interaction, (Changed<Interaction>, With<FightButton>)>,
    mut end_deployment_evt: EventWriter<EndDeployment>,
) {
    for interaction in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            end_deployment_evt.send(EndDeployment);
        }
    }
}

fn open_merchant_btn_interaction(
    q_interactions: Query<&Interaction, (Changed<Interaction>, With<OpenMerchantButton>)>,
    state: Res<State<TimeOfDay>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for interaction in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            if *state.get() == TimeOfDay::Day {
                next_game_state.set(GameState::Merchant);
            }
        }
    }
}

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

/// Pause or resumed.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum GameState {
    Merchant,
    Tavern,
    #[default]
    BuildingTurn,
    Deployment,
    BattleTurn,
    EnemyTurn,
}

#[derive(Component)]
pub struct EndTurnButton;

#[derive(Component)]
pub struct OpenMerchantButton;

/// When clicked end deployment
#[derive(Component)]
pub struct FightButton;

#[derive(Resource, Default)]
pub struct DisplayCache(EntityHashMap<Display>);

pub fn hide_all_with<T: Component>(
    mut displays: ResMut<DisplayCache>,
    mut q_vis: Query<(Entity, &mut Visibility, Option<&mut Style>), With<T>>,
) {
    for (entity, mut vis, style) in q_vis.iter_mut() {
        *vis = Visibility::Hidden;
        if let Some(mut style) = style {
            displays.0.insert(entity, style.display);
            style.display = Display::None;
        }
    }
}

pub fn show_all_with<T: Component>(
    mut displays: ResMut<DisplayCache>,
    mut q_vis: Query<(Entity, &mut Visibility, Option<&mut Style>), With<T>>,
) {
    for (entity, mut vis, style) in q_vis.iter_mut() {
        *vis = Visibility::Visible;
        if let Some(mut style) = style {
            style.display = displays.0.remove(&entity).unwrap_or(Display::DEFAULT);
        }
    }
}
