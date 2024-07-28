//! The screen state for the main game loop.

use bevy::color::palettes::css;
use bevy::ecs::entity::EntityHashMap;
use bevy::prelude::*;
use sickle_ui::prelude::*;

use super::Screen;
use crate::game::constants::{INITIAL_GOLD, INITIAL_POPULATION};
use crate::game::construction::{
    build_btn_interaction, building_panel_layout, update_build_panel, StructureCosts,
};
use crate::game::cycle::{EndDeployment, EndTurn, Season, TimeOfDay, Turn};
use crate::game::deployment::{
    deployment_setup, deployment_zone_visualization, is_deployment_ready,
};
use crate::game::events::SelectStructureTypeEvent;
use crate::game::resources::{SelectedStructueType, VillageGold, VillagePopulation};
use crate::game::unit::player::{add_starting_player_units, move_unit, reset_unit_turn_states};
use crate::game::unit::AvailableUnitNames;
use crate::game::unit_list::{
    inventory_list_layout, select_player_unit_btn_interaction, unit_list_layout,
    update_selected_unit_name_label, update_unit_list_container, PlayerUnitList,
};
use crate::game::WatchRes;
use crate::game::{assets::SoundtrackKey, audio::soundtrack::PlaySoundtrack};
use crate::modals::merchant::{exit_mechant_btn_interaction, merchant_modal_layout};
use crate::ui::interaction::apply_interaction_palette;
use crate::ui::{palette::*, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        .init_resource::<DisplayCache>()
        .init_resource::<AvailableUnitNames>()
        .init_resource::<PlayerUnitList>()
        .init_resource::<StructureCosts>()
        .init_resource::<SelectedStructueType>()
        .add_event::<SelectStructureTypeEvent>()
        .add_systems(OnEnter(Screen::Playing), enter_playing)
        .add_systems(OnEnter(Screen::Playing), add_starting_player_units)
        .add_systems(OnEnter(GameState::Merchant), merchant_modal_layout)
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
            OnEnter(GameState::Deployment),
            hide_all_with::<EndTurnButton>,
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
        );

    app.add_systems(
        Update,
        (
            // exit_btn_interaction,
            end_turn_btn_interaction,
            exit_mechant_btn_interaction,
            fight_btn_interaction,
            update_unit_list_container
                .run_if(in_state(Screen::Playing))
                .before(apply_interaction_palette),
            select_player_unit_btn_interaction,
            update_selected_unit_name_label,
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
                    .insert(WatchRes::<VillagePopulation>::default())
                    .style()
                    .font_size(LABEL_SIZE);
            });
        });
    });
}

fn enter_playing(mut commands: Commands) {
    commands.trigger(PlaySoundtrack::Key(SoundtrackKey::Gameplay));
    commands
        .ui_builder(UiRoot)
        .column(|ui| {
            ui.style()
                .width(Val::Percent(100.))
                .height(Val::Percent(100.))
                .padding(UiRect::all(Val::Px(60.)));

            // Top pane
            ui.row(|ui| {
                ui.style()
                    .justify_content(JustifyContent::SpaceBetween)
                    .align_items(AlignItems::Center)
                    .column_gap(Val::Px(40.));

                ui.label(LabelConfig::from("Season"))
                    .insert(WatchRes::<Season>::default())
                    .style()
                    .font_size(HEADER_SIZE);

                ui.column(|_| {}).style().flex_grow(1.);

                economy_status_layout(ui);

                ui.column(|_| {}).style().width(Val::Px(20.));

                // ui.container(ButtonBundle { ..default() }, |ui| {
                //     ui.label(LabelConfig::from("Pause"))
                //         .style()
                //         .font_size(LABEL_SIZE);
                // })
                // .insert((
                //     InteractionPalette {
                //         none: NODE_BACKGROUND,
                //         hovered: BUTTON_HOVERED_BACKGROUND,
                //         pressed: BUTTON_PRESSED_BACKGROUND,
                //     },
                //     // Pause button component
                //     PauseButton,
                // ))
                // .style()
                // .padding(UiRect::all(Val::Px(10.0)))
                // .border_radius(BorderRadius::all(Val::Px(5.0)));
            });
            // Center panel
            ui.row(|ui| {
                ui.style().align_items(AlignItems::Center);
                unit_list_layout(ui);
            })
            .style()
            .flex_grow(1.);
            // Bottom panel
            ui.row(inventory_list_layout);
            ui.row(|ui| {
                ui.label(LabelConfig::from("Turn Until"))
                    .insert(WatchRes::<Turn>::default())
                    .style()
                    .font_size(LABEL_SIZE);

                ui.column(|_| {}).style().flex_grow(1.);

                ui.container(ButtonBundle { ..default() }, |ui| {
                    ui.label(LabelConfig::from("End Turn"))
                        .insert(EndTurnButton)
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
                .insert(EndTurnButton)
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
        })
        .insert(StateScoped(Screen::Playing));
}

// fn resume(mut next_game_state: ResMut<NextState<GameState>>) {
//     next_game_state.set(GameState::Resumed);
// }

// fn enter_pause(mut commands: Commands) {
//     commands
//         .ui_builder(UiRoot)
//         .column(|ui| {
//             ui.style()
//                 .justify_content(JustifyContent::Center)
//                 .justify_items(JustifyItems::Center)
//                 .justify_self(JustifySelf::Center);

//             ui.row(|ui| {
//                 ui.column(|ui| {
//                     ui.label(LabelConfig::from("Paused"))
//                         .style()
//                         .font_size(40.0);

//                     ui.column(|_| {}).style().height(Val::Px(20.0));

//                     // Resume button
//                     ui.container(ButtonBundle { ..default() }, |ui| {
//                         ui.label(LabelConfig::from("Resume"))
//                             .style()
//                             .font_size(LABEL_SIZE);
//                     })
//                     .insert((
//                         InteractionPalette {
//                             none: css::GREEN.into(),
//                             hovered: css::DARK_GREEN.into(),
//                             pressed: css::GREEN.into(),
//                         },
//                         ResumeButton,
//                     ))
//                     .style()
//                     .padding(UiRect::all(Val::Px(10.0)))
//                     .border_radius(BorderRadius::all(Val::Px(5.0)));

//                     ui.column(|_| {}).style().height(Val::Px(20.0));

//                     // Exit button
//                     ui.container(ButtonBundle { ..default() }, |ui| {
//                         ui.label(LabelConfig::from("Exit"))
//                             .style()
//                             .font_size(LABEL_SIZE);
//                     })
//                     .insert((
//                         InteractionPalette {
//                             none: css::RED.into(),
//                             hovered: css::DARK_RED.into(),
//                             pressed: css::RED.into(),
//                         },
//                         ExitButton,
//                     ))
//                     .style()
//                     .padding(UiRect::all(Val::Px(10.0)))
//                     .border_radius(BorderRadius::all(Val::Px(5.0)));
//                 });
//             })
//             .style()
//             .padding(UiRect::all(Val::Px(18.0)))
//             .border_radius(BorderRadius::all(Val::Px(8.0)))
//             .background_color(Color::srgba(0.0, 0.0, 0.0, 0.6));
//         })
//         .insert(StateScoped(GameState::Paused));
// }

// fn exit_btn_interaction(
//     q_interactions: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
//     mut next_game_state: ResMut<NextState<GameState>>,
//     mut next_screen: ResMut<NextState<Screen>>,
// ) {
//     for interaction in q_interactions.iter() {
//         if let Interaction::Pressed = interaction {
//             // Resumed is default state that is needed
//             // when we go back into the game later.
//             next_game_state.set(GameState::Resumed);
//             next_screen.set(Screen::Title);
//         }
//     }
// }

fn end_turn_btn_interaction(
    q_interactions: Query<&Interaction, (Changed<Interaction>, With<EndTurnButton>)>,
    mut next_turn_evt: EventWriter<EndTurn>,
) {
    for interaction in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            next_turn_evt.send(EndTurn);
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

fn exit_playing(mut commands: Commands) {
    // We could use [`StateScoped`] on the sound playing entites instead.
    commands.trigger(PlaySoundtrack::Disable);
}

/// Pause or resumed.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub enum GameState {
    Merchant,
    #[default]
    BuildingTurn,
    Deployment,
    BattleTurn,
    EnemyTurn,
}

#[derive(Component)]
pub struct EndTurnButton;

/// When clicked end deployment
#[derive(Component)]
pub struct FightButton;

#[derive(Resource, Default)]
struct DisplayCache(EntityHashMap<Display>);

fn hide_all_with<T: Component>(
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

fn show_all_with<T: Component>(
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
