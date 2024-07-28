use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use sickle_ui::prelude::*;

use crate::game::MERCHANT_ITEMS;
use crate::game::MERCHANT_Z_LAYER;
use crate::screen::playing::GameState;
use crate::ui::palette::LABEL_SIZE;
use crate::ui::prelude::InteractionPalette;

#[derive(Component, Default)]
pub struct BuyButton(pub Option<Entity>);

#[derive(Component)]
pub struct ItemButton(pub Entity);

#[derive(Component)]
pub struct ExitMerchantButton;

pub fn merchant_modal_layout(mut commands: Commands) {
    commands
        .ui_builder(UiRoot)
        .column(|ui| {
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
                    ui.icon("icons/shop_character.png")
                        .style()
                        .width(Val::Px(192.))
                        .height(Val::Px(192.));
                    ui.label(LabelConfig::from("Buy anything you like!"))
                        .style()
                        .margin(UiRect::all(Val::Px(16.)));

                    ui.row(|ui| {
                        ui.style().column_gap(Val::Px(16.));
                        for _ in 0..MERCHANT_ITEMS {
                            ui.container(ButtonBundle { ..default() }, |ui| {
                                ui.style().border_radius(BorderRadius::all(Val::Px(16.)));
                                ui.column(|ui| {
                                    ui.style()
                                        .align_items(AlignItems::Center)
                                        .justify_content(JustifyContent::Center)
                                        .width(Val::Px(128.))
                                        .height(Val::Px(128.));
                                    ui.label(LabelConfig::from("ITEM"))
                                        .style()
                                        .font_size(LABEL_SIZE);
                                });
                            })
                            .insert((
                                InteractionPalette {
                                    none: css::GREEN.into(),
                                    hovered: css::DARK_GREEN.into(),
                                    pressed: css::GREEN.into(),
                                },
                                ItemButton(Entity::PLACEHOLDER),
                            ));
                        }
                    });

                    ui.row(|ui| {
                        ui.style()
                            .background_color(css::MAROON.into())
                            .margin(UiRect::all(Val::Px(30.)))
                            .flex_grow(1.)
                            .align_items(AlignItems::Center)
                            .justify_content(JustifyContent::Center);
                        ui.label(LabelConfig::from("ITEM DESCRIPTION"));
                    });

                    // Buy button
                    ui.container(ButtonBundle::default(), |ui| {
                        ui.label(LabelConfig::from("Buy for 100 gold"))
                            .style()
                            .font_size(LABEL_SIZE);
                    })
                    .insert((
                        InteractionPalette {
                            none: css::GREEN.into(),
                            hovered: css::DARK_GREEN.into(),
                            pressed: css::GREEN.into(),
                        },
                        BuyButton::default(),
                    ));

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
                        ExitMerchantButton,
                    ))
                    .style()
                    .position_type(PositionType::Absolute)
                    .top(Val::Px(16.))
                    .right(Val::Px(16.));
                });
            });
        })
        .insert(StateScoped(GameState::Merchant))
        .style()
        .focus_policy(FocusPolicy::Block)
        .z_index(ZIndex::Global(MERCHANT_Z_LAYER))
        .width(Val::Percent(100.))
        .height(Val::Percent(100.))
        .background_color(Color::srgba(0.25, 0.25, 0.25, 0.75))
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center);
}

pub fn exit_merchant() {}

pub fn exit_mechant_btn_interaction(
    q_interactions: Query<&Interaction, (Changed<Interaction>, With<ExitMerchantButton>)>,
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
