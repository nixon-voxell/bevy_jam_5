use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use sickle_ui::prelude::*;

use crate::game::inventory::MaxInventorySize;
use crate::game::unit::Health;
use crate::game::unit::Movement;
use crate::game::unit::PlayerUnit;
use crate::game::unit::UnitName;
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
pub enum TavernButton {
    Select(Entity),
}

/// Each can be bought twice
#[derive(Component)]
pub enum TavernUpgrade {
    AddMovement,
    AddHealth,
    AddItemSlot,
}

pub fn tavern_modal_layout(
    mut commands: Commands,
    mut unit_query: Query<
        (
            Entity,
            &UnitName,
            &mut Movement,
            &mut Health,
            &mut MaxInventorySize,
        ),
        With<PlayerUnit>,
    >,
) {
    commands.ui_builder(UiRoot).column(|ui| {
        ui.row(|ui| {
            ui.style()
                .padding(UiRect::all(Val::Px(18.)))
                .border(UiRect::all(Val::Px(2.)))
                .border_color(Color::WHITE)
                .border_radius(BorderRadius::all(Val::Px(16.)))
                .background_color(Color::BLACK.with_alpha(0.8))
                .width(Val::Px(480.))
                .height(Val::Px(600.))
                .justify_content(JustifyContent::Center);
            ui.column(|ui| {
                ui.style()
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::SpaceBetween);
                ui.label(LabelConfig::from("Tavern"))
                    .style()
                    .margin(UiRect::all(Val::Px(16.)));
                ui.icon("icons/mug_of_beer.png")
                    .style()
                    .width(Val::Px(64.))
                    .height(Val::Px(64.));

                ui.row(|ui| {
                    ui.column(|ui| {
                        for (entity, name, _, _, _) in unit_query.iter_mut() {
                            ui.row(|ui| {
                                ui.insert(TavernButton::Select(entity));

                                ui.style()
                                    .margin(UiRect::all(Val::Px(2.)))
                                    .border(UiRect::all(Val::Px(2.)))
                                    .border_color(Color::WHITE)
                                    .justify_content(JustifyContent::Start);
                                ui.label(LabelConfig::from(name.0.clone()));
                            });
                        }
                    });

                    ui.column(|ui| {
                        for (upgrade, desc) in [
                            (TavernUpgrade::AddMovement, "Add movement"),
                            (TavernUpgrade::AddHealth, "Add health"),
                            (TavernUpgrade::AddItemSlot, "Add item slot"),
                        ] {
                            ui.container(ButtonBundle::default(), |ui| {
                                ui.insert(upgrade)
                                    .style()
                                    .margin(UiRect::all(Val::Px(2.)))
                                    .border(UiRect::all(Val::Px(2.)))
                                    .border_color(Color::WHITE)
                                    .justify_content(JustifyContent::Start);
                                ui.label(LabelConfig::from(desc));
                            });
                        }
                    });
                });
            });
        });
    });
}

pub fn exit_tavern_btn_interaction(
    q_interactions: Query<&Interaction, (Changed<Interaction>, With<ExitTavernButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for interaction in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            // Resumed is default state that is needed
            // when we go back into the game later.
            next_game_state.set(GameState::BuildingTurn);
        }
    }
}
