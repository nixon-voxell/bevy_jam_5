use bevy::color::palettes::css;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use sickle_ui::prelude::*;

use crate::game::actors_list::SellItemButton;
use crate::game::assets::SfxKey;
use crate::game::audio::sfx::PlaySfx;
use crate::game::constants::TEXT_SIZE;
use crate::game::inventory::Inventory;
use crate::game::inventory::Item;
use crate::game::inventory::ITEM_TEMPLATES;
use crate::game::resources::VillageGold;
use crate::game::selection::SelectedActor;
use crate::game::MODAL_Z_LAYER;
use crate::screen::playing::hide_all_with;
use crate::screen::playing::show_all_with;
use crate::screen::playing::GameState;
use crate::ui::icon_set::IconSet;
use crate::ui::palette::HEADER_SIZE;
use crate::ui::prelude::InteractionPalette;

const PLACEHOLDER_DESCRIPTION: &str = "Select an item to view its description here.";

pub struct MerchantModalPlugin;

impl Plugin for MerchantModalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MerchantItems>()
            .add_systems(
                OnEnter(GameState::Merchant),
                (merchant_modal_layout, show_all_with::<SellItemButton>),
            )
            .add_systems(OnExit(GameState::Merchant), hide_all_with::<SellItemButton>)
            .add_systems(
                Update,
                (
                    exit_mechant_btn_interaction,
                    sell_btn_interaction,
                    (
                        item_btn_interaction,
                        item_btn_highlight.run_if(resource_changed::<MerchantItems>),
                    )
                        .chain(),
                    buy_btn_interaction,
                ),
            );
    }
}

#[derive(Component)]
pub struct BuyButton;

#[derive(Component)]
pub struct ItemButton(pub usize);

#[derive(Component)]
pub struct DescriptionLabel;

#[derive(Component)]
pub struct CostLabel;

#[derive(Component)]
pub struct ExitMerchantButton;

#[derive(Component)]
pub struct SellButton;

#[derive(Component)]
pub struct ItemBorder(pub usize);

#[derive(Resource, Default, Debug)]
pub struct MerchantItems {
    pub items: [Option<&'static Item>; 3],
    pub selection: Option<usize>,
}

fn merchant_modal_layout(
    mut commands: Commands,
    icon_set: Res<IconSet>,
    mut merchant_items: ResMut<MerchantItems>,
) {
    if merchant_items.items.iter().any(|item| item.is_none()) {
        for item in merchant_items.items.iter_mut() {
            let index = rand::random::<usize>() % ITEM_TEMPLATES.len();
            *item = Some(&ITEM_TEMPLATES[index]);
        }
    }

    commands
        .ui_builder(UiRoot)
        .column(|ui| {
            ui.row(|ui| {
                ui.style()
                    .padding(UiRect::all(Val::Px(18.)))
                    .border(UiRect::all(Val::Px(2.)))
                    .border_color(Color::WHITE)
                    .background_color(Color::BLACK.with_alpha(0.8))
                    .width(Val::Vh(80.))
                    .height(Val::Vh(80.))
                    .justify_content(JustifyContent::Center);

                ui.column(|ui| {
                    ui.style()
                        .align_items(AlignItems::Center)
                        .justify_content(JustifyContent::Center);

                    ui.icon("icons/shop_character.png")
                        .style()
                        .width(Val::Vh(12.0))
                        .height(Val::Vh(12.0))
                        .margin(UiRect::all(Val::Px(6.0)));

                    ui.row(|ui| {
                        ui.style()
                            .column_gap(Val::Px(16.))
                            .justify_content(JustifyContent::Center)
                            .padding(UiRect::all(Val::Px(6.0)));

                        for (i, item) in merchant_items.items.iter().enumerate() {
                            let mut image = UiImage::default();
                            if let Some(item) = item {
                                image.texture = icon_set.get(item.name);
                            }

                            ui.container(ButtonBundle { image, ..default() }, |_| {})
                                .insert((
                                    InteractionPalette {
                                        none: Color::BLACK,
                                        hovered: Color::BLACK.lighter(0.4),
                                        pressed: Color::BLACK.lighter(0.2),
                                    },
                                    ItemButton(i),
                                ))
                                .style()
                                .border(UiRect::all(Val::Px(2.0)))
                                .border_color(Color::WHITE)
                                .border_radius(BorderRadius::all(Val::Px(8.0)))
                                .align_items(AlignItems::Center)
                                .justify_content(JustifyContent::Center)
                                .padding(UiRect::all(Val::Px(4.0)))
                                .width(Val::Vh(10.))
                                .height(Val::Vh(10.));
                        }
                    });

                    ui.row(|ui| {
                        ui.label(LabelConfig::from(PLACEHOLDER_DESCRIPTION))
                            .insert(DescriptionLabel)
                            .style()
                            .font_size(TEXT_SIZE);
                    })
                    .style()
                    .background_color(Color::BLACK)
                    .padding(UiRect::all(Val::Px(8.0)))
                    .margin(UiRect::all(Val::Px(6.0)))
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center);

                    // Buy button
                    ui.container(ButtonBundle::default(), |ui| {
                        ui.label(LabelConfig::from("Select an item to buy."))
                            .insert(CostLabel)
                            .style()
                            .font_size(TEXT_SIZE);
                    })
                    .insert((
                        InteractionPalette {
                            none: css::GOLD.darker(0.5).into(),
                            hovered: css::LIGHT_GOLDENROD_YELLOW.darker(0.7).into(),
                            pressed: css::GOLD.darker(0.3).into(),
                        },
                        BuyButton,
                    ))
                    .style()
                    .padding(UiRect::all(Val::Px(6.0)));
                });

                // Close button
                ui.container(ButtonBundle::default(), |ui| {
                    ui.label(LabelConfig::from("Close"))
                        .style()
                        .font_size(HEADER_SIZE)
                        .font_color(css::RED.into());
                })
                .insert(ExitMerchantButton)
                .style()
                .position_type(PositionType::Absolute)
                .top(Val::Px(16.))
                .right(Val::Px(26.));
            });
        })
        .insert(StateScoped(GameState::Merchant))
        .style()
        .focus_policy(FocusPolicy::Block)
        .z_index(ZIndex::Global(MODAL_Z_LAYER))
        .width(Val::Percent(100.))
        .height(Val::Percent(100.))
        .background_color(Color::srgba(0.25, 0.25, 0.25, 0.75))
        .justify_content(JustifyContent::Center)
        .align_items(AlignItems::Center);
}

fn item_btn_highlight(
    mut q_interactions: Query<(&ItemButton, &mut BorderColor)>,
    merchant_items: Res<MerchantItems>,
) {
    for (button, mut border_col) in q_interactions.iter_mut() {
        if merchant_items.selection == Some(button.0) {
            border_col.0 = css::YELLOW.into();
        } else {
            border_col.0 = Color::WHITE;
        }
    }
}

fn item_btn_interaction(
    mut q_interactions: Query<(&Interaction, &ItemButton), Changed<Interaction>>,
    mut q_cost_label: Query<&mut Text, (With<CostLabel>, Without<DescriptionLabel>)>,
    mut q_description_label: Query<&mut Text, (With<DescriptionLabel>, Without<CostLabel>)>,
    mut merchant_items: ResMut<MerchantItems>,
) {
    for (interaction, button) in q_interactions.iter_mut() {
        let (Ok(mut cost), Ok(mut description)) = (
            q_cost_label.get_single_mut(),
            q_description_label.get_single_mut(),
        ) else {
            return;
        };

        if let Interaction::Pressed = interaction {
            merchant_items.selection = Some(button.0);

            let item = merchant_items.items[button.0];
            if let Some(item) = item {
                cost.sections[0] = TextSection::new(
                    format!("Buy for {} coin(s).", item.cost),
                    TextStyle {
                        font_size: TEXT_SIZE,
                        ..default()
                    },
                );
                description.sections[0] = TextSection::new(
                    item.description,
                    TextStyle {
                        font_size: TEXT_SIZE,
                        ..default()
                    },
                );
            }
        }
    }
}

fn buy_btn_interaction(
    mut commands: Commands,
    selected_unit: Res<SelectedActor>,
    mut iq: Query<&mut Inventory>,
    q_interactions: Query<&Interaction, (Changed<Interaction>, With<BuyButton>)>,
    mut merchant_items: ResMut<MerchantItems>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut gold: ResMut<VillageGold>,
) {
    let Some(entity) = selected_unit.entity else {
        return;
    };
    let Ok(mut inventory) = iq.get_mut(entity) else {
        return;
    };
    let Some(slot) = inventory.get_empty_slot() else {
        return;
    };

    for interaction in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            if let Some(selected_item) = merchant_items
                .selection
                .and_then(|i| merchant_items.items[i].take())
            {
                if selected_item.cost <= gold.0 {
                    gold.0 = gold.0.saturating_sub(selected_item.cost);
                    inventory.set(slot, *selected_item);
                    next_game_state.set(GameState::BuildingTurn);
                }
            }

            commands.trigger(PlaySfx::Key(SfxKey::CoinPurchase));
        }
    }
}

fn sell_btn_interaction(
    mut commands: Commands,
    selected: Res<SelectedActor>,
    q_interactions: Query<&Interaction, (Changed<Interaction>, With<SellItemButton>)>,
    mut iq: Query<&mut Inventory>,
    mut gold: ResMut<VillageGold>,
) {
    let Some(entity) = selected.entity else {
        return;
    };
    let Ok(mut inventory) = iq.get_mut(entity) else {
        return;
    };

    let Some(i) = inventory.selected_item else {
        return;
    };

    for interaction in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            if let Some(item) = inventory.take(i) {
                gold.0 += item.cost / 2;
            }

            commands.trigger(PlaySfx::Key(SfxKey::CoinPurchase));
        }
    }
}

fn exit_mechant_btn_interaction(
    q_interactions: Query<&Interaction, (Changed<Interaction>, With<ExitMerchantButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for interaction in q_interactions.iter() {
        if let Interaction::Pressed = interaction {
            next_game_state.set(GameState::BuildingTurn);
        }
    }
}
